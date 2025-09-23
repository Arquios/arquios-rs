use std::str::FromStr;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub const MESES_MINI: [&str; 12] = ["Ene", "Feb", "Mar", "Abr", "May", "Jun", "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"];
//const DS_MINI: [&str; 7] = ["Lun", "Mar", "Mie", "Jue", "Vie", "Sab", "Dom"];

// FORMATOS: https://docs.rs/chrono/0.4.15/chrono/format/strftime/index.html#specifiers

/// Fecha y/u Hora actual
///
/// # Ejemplo
///
/// ```
/// let fecha = ahora("%F");
/// let hora = ahora("%T");
/// let fechahora = ahora("%F %T");
/// ```
///
pub fn ahora(formato: &str) -> String {
	Local::now().format(formato).to_string()
}

pub fn ahora_log() -> String {
	Local::now().format("[%F %T] ").to_string()
}

pub trait FormatosFecha {
	fn to_string_ddmmmaa(&self) -> String;
}

impl FormatosFecha for NaiveDate {
	fn to_string_ddmmmaa(&self) -> String {
		format!("{} {} {}", self.day(), MESES_MINI[(self.month() - 1) as usize], self.year())
	}
}

pub fn fecha_str_ddmmmaa(f: &String) -> String {
	if f == "" {
		return "-------".to_owned();
	}
	match NaiveDate::from_str(f) {
		Ok(f) => f.to_string_ddmmmaa(),
		Err(_) => "-------".to_string()
	}
}

pub trait FormatosHora {
	fn to_string_hhmm(&self) -> String;
	fn to_string_hhmmss(&self) -> String;
	fn to_string_hhmm_ampm(&self) -> String;
}

impl FormatosHora for NaiveTime {
	fn to_string_hhmm(&self) -> String {
		self.format("%H:%M").to_string()
	}
	fn to_string_hhmmss(&self) -> String {
		self.format("%H:%M:%S").to_string()
	}
	fn to_string_hhmm_ampm(&self) -> String {
		self.format("%-I:%M %p").to_string()
	}
}

pub fn hora_hhmm(t: &String) -> String {
	NaiveTime::from_str(t).map_or("---".to_string(), |t|t.to_string_hhmm())
}
pub fn hora_hhmm_ampm(t: &String) -> String {
	NaiveTime::from_str(t).map_or("---".to_string(), |t|t.to_string_hhmm_ampm())
}


pub trait FormatosFechaHora {
	fn to_string_ddmmmaa_hhmm(&self) -> String;
	fn to_string_ddmmmaa_hhmmss(&self) -> String;
}

impl FormatosFechaHora for DateTime<Local> {
	fn to_string_ddmmmaa_hhmm(&self) -> String {
		format!("{}  {}", self.date_naive().to_string_ddmmmaa(), self.time().to_string_hhmm())
	}
	fn to_string_ddmmmaa_hhmmss(&self) -> String {
		format!("{}  {}", self.date_naive().to_string_ddmmmaa(), self.time().to_string_hhmmss())
	}
}

impl FormatosFechaHora for NaiveDateTime {
	fn to_string_ddmmmaa_hhmm(&self) -> String {
		format!("{}  {}", self.date().to_string_ddmmmaa(), self.time().to_string_hhmm())
	}
	fn to_string_ddmmmaa_hhmmss(&self) -> String {
		format!("{}  {}", self.date().to_string_ddmmmaa(), self.time().to_string_hhmmss())
	}
}

pub trait FormatosAntiguedad {
	fn d_h_m_uno(&self) -> String;
	fn h_m_junto(&self) -> String;
	fn h_m_s_tiempo(&self) -> String;
	fn m_s_tiempo(&self) -> String;
	fn m(&self) -> String;
	fn s_float(&self) -> String;
}
impl FormatosAntiguedad for Duration {
	fn d_h_m_uno(&self) -> String {
		if self.num_days() >= 1 { format!("{} d", self.num_days()) }
		else if self.num_hours() >= 1 { format!("{} h", self.num_hours()) }
		else { format!("{} m", self.num_minutes()) }
	}
	fn h_m_junto(&self) -> String {
		let minutos = self.num_minutes();
		let sig_neg = if minutos.is_negative() { "-" } else { "" };
		let horas = self.num_hours().abs();
		let minutos = minutos.abs();
		if horas == 0 {
			format!("{}{}m", sig_neg, minutos)
		} else {
			format!("{}{}h {}m", sig_neg, horas, minutos - (horas * 60))
		}
	}
	fn h_m_s_tiempo(&self) -> String {
		let segundos = self.num_seconds();
		let sig_neg = if segundos.is_negative() { "-" } else { "" };
		let horas = self.num_hours().abs();
		let minutos = self.num_minutes().abs();
		let segundos = segundos.abs() - minutos * 60;
		if horas == 0 {
			format!("{}{}:{:0>2}", sig_neg, minutos, segundos)
		} else {
			format!("{}{}:{:0>2}:{:0>2}", sig_neg, horas, minutos - (horas * 60), segundos)
		}
	}
	fn m_s_tiempo(&self) -> String {
		let minutos = self.num_minutes();
		format!("{}:{:0>2}", minutos, self.num_seconds() - (minutos * 60))
	}
	fn m(&self) -> String {
		format!("{}m", self.num_minutes())
	}

	fn s_float(&self) -> String {
		format!("{:3} seg.", self.num_milliseconds() as f64 / 1000.0)
	}
}

pub mod json_date_time {

	use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
	use chrono::naive::NaiveDateTime;

	pub fn serialize<S>(
		dt: &NaiveDateTime,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		dt.format("%Y-%m-%d %H:%M:%S").to_string().serialize(serializer)
	}

	pub fn deserialize<'de, D>(
		deserializer: D,
	) -> Result<NaiveDateTime, D::Error>
	where
		D: Deserializer<'de>,
	{
		let t = String::deserialize(deserializer)?;

		// it doesn't try to handle the error, just unwraps
		NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S").map_err(|e| Error::custom(format!("Formato DateTime incorrecto: {e}")) )
	}

}
pub mod json_date_time_op {

	use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
	use chrono::naive::NaiveDateTime;

	pub fn serialize<S>(
		dt: &Option<NaiveDateTime>,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		dt.map(|dt|dt.format("%Y-%m-%d %H:%M:%S").to_string()).serialize(serializer)
	}

	pub fn deserialize<'de, D>(
		deserializer: D,
	) -> Result<Option<NaiveDateTime>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let t = Option::<String>::deserialize(deserializer)?;
		if let Some(t) = t {
			Ok(Some(NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S")
				.map_err(|e| Error::custom(format!("Formato DateTime incorrecto: {e}")))?))
		} else {
			Ok(None)
		}
	}

}


#[derive(Serialize,Deserialize,Clone)]
pub struct RangoFechas {
	pub de: NaiveDate,
	pub a: NaiveDate
}
#[derive(Serialize,Deserialize,Clone)]
pub struct RangoHoras {
	pub de: NaiveTime,
	pub a: NaiveTime
}
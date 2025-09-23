use std::sync::{Arc, Mutex, MutexGuard};
use rust_decimal::Decimal;

pub const DEC_0: Decimal = Decimal::ZERO;
pub const DEC_1: Decimal = Decimal::from_parts(1, 0, 0, false, 0);
pub const DEC_1_NEG: Decimal = Decimal::from_parts(1, 0, 0, true, 0);
pub const DEC_100: Decimal = Decimal::from_parts(100, 0, 0, false, 0);
pub const DEC_0_01: Decimal = Decimal::from_parts(1, 0, 0, false, 2);
pub const DEC_0_001: Decimal = Decimal::from_parts(1, 0, 0, false, 3);


pub type ArcMutex<T> = Arc<Mutex<T>>;

pub trait ParaArcMutex<T> {
	fn nuevo(t: T) -> ArcMutex<T>;
	fn nuevo_default() -> ArcMutex<T> where T: Default;
	fn bloquear(&self) -> MutexGuard<T>;
}

impl <T>ParaArcMutex<T> for ArcMutex<T> {
	fn nuevo(t: T) -> ArcMutex<T> {
		Arc::new(Mutex::new(t))
	}
	fn nuevo_default() -> ArcMutex<T> where T: Default {
		Arc::new(Mutex::new(Default::default()))
	}
	fn bloquear(&self) -> MutexGuard<T> {
		self.lock().unwrap_or_else(|e|e.into_inner())
	}
}



struct SegmentosVersionado {
	a: u32,
	b: u32,
	c: u32,
}
fn versionado_a_segmentos(version: &str) -> SegmentosVersionado {
	let mut partes = version.split('.');
	SegmentosVersionado {
		a: partes.next().unwrap_or_default().parse().unwrap_or_default(),
		b: partes.next().unwrap_or_default().parse().unwrap_or_default(),
		c: partes.next().unwrap_or_default().parse().unwrap_or_default(),
	}
}

/// Determina si la versión nueva es superior a la actual
pub fn version_superior(nueva: &str, actual: &str) -> bool {
	//println!("COMPARANDO {} vs {}", nueva, actual);
	if nueva.is_empty() || actual.is_empty() || actual == nueva {
		return false;
	}

	let nueva = versionado_a_segmentos(nueva);
	let actual = versionado_a_segmentos(actual);

	if nueva.a > actual.a {
		return true;
	}
	if nueva.a == actual.a && nueva.b > actual.b {
		return true;
	}
	if nueva.a == actual.a && nueva.b == actual.b && nueva.c > actual.c {
		return true;
	}

	false
}


/// Otra forma de verificar que ningún otra instancia o proceso esté corriendo......, probé:
/// 	- bloquear archivo.... si bueno... no funciona si lo corres desde otra carpeta...
/// 	- listado de procesos.... bueno necesitas el permiso para ver los procesos...
/// 	- compartir variable en memoria... no me funcionó en algunos sistemas....
pub fn instancia_unica() {
	match std::net::TcpListener::bind("0.0.0.0:6427") {
		Ok(listener) => {
			#[cfg(debug_assertions)]
			println!("{} Validación de instancia correcta", crate::librerias::tiempo::ahora_log());
			std::thread::spawn(move || {
				for _stream in listener.incoming() { //PUNTO DE ESPERA PARA CONEXIONES ENTRANTES
					//Desechamos por que este puerto solo tiene el fin de ser apartado, no se procesará nada.....
				}
				#[cfg(debug_assertions)]
				println!("{} Se acabó validación de instancia", crate::librerias::tiempo::ahora_log());
			});
		}
		Err(e) => {
			println!("Ya existe una instancia del programa ejecutándose ({e})");
			std::process::exit(2)
		}
	}
}
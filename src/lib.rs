pub mod tiempo;
pub mod utilidades;
pub mod ws;

#[macro_use]
extern crate serde_derive;

pub type Resultado<T> = Result<T, String>;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::tiempo::{Diccionario, FormatosFecha};
    use crate::ws::peticion_raw_get;

    #[test]
    fn fecha_diccionario() {
        const MESES: [&str; 12] = [ "Ene", "Feb", "Mar", "Abr", "May", "Jun", "Jul", "Ago", "Sep", "Oct", "Nov", "Dic" ];

        struct SistemaDiccionario {}
        impl Diccionario for SistemaDiccionario {
            fn traduccion_mes_abr(&self, mes: u32) -> &str {
                MESES[mes as usize - 1]
            }
        }
        let diccionario = SistemaDiccionario {};
        let fecha = NaiveDate::from_ymd_opt(2015, 7, 22).unwrap();
        println!("{:?} {}", fecha, fecha.to_string_ddmmmaa(diccionario));
    }

    #[test]
    fn ws() {

        match peticion_raw_get("https://google.com", 300) {
            Ok(r) => {
                println!("WS: {:?}", r);
            }
            Err(e) => {
                println!("Error WS: {e}");
            }
        }

    }
}

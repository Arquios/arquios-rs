pub mod tiempo;
pub mod utilidades;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::tiempo::{Diccionario, FormatosFecha};

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
}

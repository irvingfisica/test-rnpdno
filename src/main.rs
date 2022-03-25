use chrono::prelude::*;
use reqrnpdno::{extractora,parameters::Parametros,utilidades};
use extractora::Iteradora;
use std::time::Instant;

fn main () {

    let limite_inferior = NaiveDate::from_ymd(2006, 1, 1);
    let limite_superior = NaiveDate::from_ymd(2022, 3, 24);

    let mut fecha_iter = limite_superior.clone();

    let variables = Iteradora::new();

    let mut start = Instant::now();

    while fecha_iter >= limite_inferior {
        let fecha = fecha_iter.format("%Y-%m-%d").to_string();

        let ruta_salida = utilidades::crear_directorio("./datos/", &fecha).unwrap();

        let mut parametros = Parametros::new();

        parametros.id_estatus_victima = "7".to_string();
        parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
        parametros.fecha_inicio = fecha.to_string();
        parametros.fecha_fin = fecha.to_string();

        match extractora::extraer_iterando(&parametros, &ruta_salida, &variables) {
            Ok(()) => {
                fecha_iter = fecha_iter.pred();
                let duration = start.elapsed();
                println!("fecha: {}, tiempo: {:?}", fecha, duration);
                start = Instant::now();
            },
            _ => {
                println!("Falló intento en fecha: {}", fecha);
            }
        }

        
    }

}
//! Módulo para manejar la configuración de la aplicación,
//! cargando los parámetros desde variables de entorno.
use dotenvy::dotenv;
use std::env;

/// Estructura que almacena los parámetros de configuración del algoritmo WOA.
#[derive(Debug)]
pub struct Config {
    pub num_drops : usize,
    pub max_iterations: usize,
    pub erosion_rate_k : f64,
    pub sedimentation_rate : f64,
    pub initial_altitude : f64
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let num_drops = env::var("NUM_DROPS")
            .expect("Falta NUM_DROPS en .env")
            .parse::<usize>()
            .expect("NUM_DROPS debe ser un número entero");

        let max_iterations = env::var("MAX_ITERATION")
            .expect("Falta MAX_ITERATION en .env")
            .parse::<usize>()
            .expect("MAX_ITERATION debe ser un número entero");

        let erosion_rate_k = env::var("EROSION_RATE_K")
            .expect("Falta EROSION_RATE_K en .env")
            .parse::<f64>()
            .expect("EROSION_RATE_K debe ser un número");

        let sedimentation_rate = env::var("SEDIMENTATION_RATE")
            .expect("Falta SEDIMENTATION_RATE en .env")
            .parse::<f64>()
            .expect("SEDIMENTATION_RATE debe ser un número");

        let initial_altitude = env::var("INITIAL_ALTITUDE")
            .expect("Falta INITIAL_ALTITUDE en .env")
            .parse::<f64>()
            .expect("INITIAL_ALTITUDE debe ser un número");

        Config {
            num_drops,
            max_iterations,
            erosion_rate_k,
            sedimentation_rate,
            initial_altitude
        }
    }
}
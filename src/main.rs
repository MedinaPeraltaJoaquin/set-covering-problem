mod utils;
mod entity;

use std::{env, process::exit};
use chrono::Local;

use utils::read_input::ReadInput;
use entity::set::Set;
use entity::subset_cover::SubsetCover;

pub fn main(){
    let args : Vec<String> = env::args().collect();
    let mut read_input = match ReadInput::new(args) {
        Ok(read) => read,
        Err(e) => {
            panic!("Error al leer argumentos: {:?}\nUtilice --help o -h", e);
        }
    };

    if read_input.get_help() {
        read_input.print_help();
        exit(0);
    }

    println!("Iniciando SCP con River Formation Dynamics (RFD)");
    let verbose_mode = read_input.get_verbose();
    let svg_mode = read_input.get_svg();

    let seeds = match read_input.get_seed() {
        Ok(seeds) => seeds,
        Err(e) => {
            panic!("Error al leer la semilla: {:?}",e);
        }
    };

    println!("Cargando subconjuntos del archivo");
    let subset_vec = match read_input.read_file_subset(){
        Ok(result) => result,
        Err(e) => {
            panic!("{:?}",e);
        }
    };

    let set = Set::new(subset_vec);
    println!("{}",set);
}
mod utils;
mod entity;
mod rfd;

use std::time::Instant;
use std::{env, process::exit};
use chrono::Local;

use utils::write_report::WriteReport;
use utils::read_input::ReadInput;
use utils::config::Config;
use utils::svg_plot::plot_convergence;

use entity::set::Set;
use rfd::river_formation_dynamics::RiverFormationDynamics;


pub fn main(){
    let args : Vec<String> = env::args().collect();
    let filename = "subsets";
    let folder = "reports";

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

    let config = Config::from_env();
    let mut best_solution = std::f64::INFINITY;
    let mut best_seed = seeds[0];

    let set = Set::new(subset_vec);
    println!("U: {}",set.get_size_set());

    let size_seeds = seeds.len();
    let start = Instant::now();
    for seed in seeds {
        let start_s = Instant::now();
        println!("Running RFD with seed: {}",seed);
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S-%3f").to_string();
        let mut river_algorithm = RiverFormationDynamics::new(
            config.num_drops, 
            config.max_iterations, 
            config.erosion_rate_k, 
            config.sedimentation_rate, 
            config.initial_altitude,
            set.get_size_subsets(), 
            seed as u64
        );

        let mut best_cover = river_algorithm.solve(&set,verbose_mode);
        let costs = river_algorithm.get_convergence_curve();
        let cost = best_cover.get_cost(&set);

        println!("C: {}",cost);
        println!("E: {:?}",best_cover.elements);
        println!("S: {}",best_cover.size);
        println!("T: {:?}",start_s.elapsed());
        println!("Disjoint: {}",set.count_disjoint_subsets(&best_cover.elements));

        if !set.is_cover_valid(&best_cover.elements) {
            eprintln!("Warning: The best RFD cover is inconsistent!");
        }
        
        let report = WriteReport::new(best_cover, timestamp.clone());
        match report.save_result_subsets(filename, folder, &set,seed) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error al guardar el archivo: {}", e);
                exit(1);
            }
        };

        if svg_mode {
            let filename_plot = format!("convergence_seed_{}_{}.svg",seed,timestamp.clone());
            match plot_convergence(&costs, &filename_plot) {
                Ok(_) => println!("Gráfica de convergencia guardada en: {}", filename_plot),
                Err(e) => eprintln!("Error al guardar la gráfica de convergencia: {}", e),
            };
        }


        if cost < best_solution {
            best_solution = cost;
            best_seed = seed;
        }
    }
    let duration = start.elapsed();
    println!("Tiempo para procesar {} semillas: {:?}",size_seeds,duration);
    println!("Mejor solución encontrada con semilla {}: Costo = {}", best_seed, best_solution);

    exit(0);
}
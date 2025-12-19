mod utils;

use std::{env, process::exit};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use rand::rngs::StdRng;
use utils::read_input::Args;
use chrono::Local;

fn save_subsets(subsets : &Vec<Vec<i32>>, filename : &str, folder_name : &str) -> io::Result<()> {
    let now = Local::now();
    let timestamp_str = now.format("%Y%m%d_%H%M%S").to_string();
    
    let dir_path = Path::new(folder_name);
    let file_name = format!("{}.{}.txt", filename,timestamp_str);
    let full_path = dir_path.join(file_name);
    
    fs::create_dir_all(dir_path)?;
    
    let mut output_content = String::new();
    for subset in subsets {
        let line = subset.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(",");

        output_content.push_str(&line);
        output_content.push('\n');
    }

    let mut file = File::create(&full_path)?; 

    file.write_all(output_content.as_bytes())?;

    println!("Datos guardados exitosamente en '{}'", full_path.display());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = "subset_input";
    let folder_name = "input";

    let mut read_input : Args = Args::new(args);

    if read_input.get_help() {
        read_input.print_help();
        exit(0);
    }

    let number_universe = read_input.get_number_universe();

    if number_universe < 2 {
        panic!("Debe de haber más de dos elementos en el universo");
    }
    let number_subset = read_input.get_number_subset();
    let seed = read_input.get_seed();

    let mut subsets : Vec<Vec<i32>> = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed as u64);

    println!("Generando subsets");
    while subsets.len() < number_subset as usize {
        let max_size = (number_universe / 10)  as usize;
        let number_values: usize = rng.gen_range(1..=max_size);
        let probabilty = rng.gen_range(0.0..1.0);
        if probabilty > 0.4 {
            continue;
        }

        let mut unique_values: HashSet<i32> = HashSet::with_capacity(number_values);
        while unique_values.len() < number_values {
            let random_value = rng.gen_range(1..=number_universe);
            
            unique_values.insert(random_value); 
        }

        let nuevo_subset: Vec<i32> = unique_values.into_iter().collect();

        subsets.push(nuevo_subset);
    }

    let mut covered_elements: HashSet<i32> = HashSet::new();
    for subset in &subsets {
        for &element in subset {
            covered_elements.insert(element);
        }
    }

    let all_universe_elements: HashSet<i32> = (1..=number_universe).collect();
    
    let mut missing_elements: Vec<i32> = all_universe_elements
        .difference(&covered_elements) 
        .cloned() 
        .collect();

    if !missing_elements.is_empty() {
        
        for element in missing_elements.drain(..) { // drain consume el vector, moviendo los elementos
            let subset_index = rng.gen_range(0..subsets.len());
            let target_subset = &mut subsets[subset_index];
            
            let mut temp_set: HashSet<i32> = target_subset.iter().cloned().collect();
            
            if temp_set.insert(element) {
                *target_subset = temp_set.into_iter().collect();
            }
        }
    }


    println!("Guardando archivo");
    match save_subsets(&subsets, filename,folder_name) {
        Ok(_) => {}, // Todo bien
        Err(e) => {
            eprintln!("Error al guardar el archivo: {}", e);
            exit(1);
        }
    }
}
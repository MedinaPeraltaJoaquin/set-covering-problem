pub struct Args {
    args : Vec<String>,
    number_universe : i32,
    number_subset : i32,
    seed : i32
}

impl Args {
    pub fn new(args : Vec<String>) -> Self {
        Args { 
            args, 
            number_universe: i32::MAX, 
            number_subset: i32::MAX, 
            seed: i32::MAX 
        }
    }

    pub fn get_seed(&mut self) -> i32 {
        if self.seed < i32::MAX {
            return self.seed;
        }

        let pos_s = self.get_position_flag("-s");

        match pos_s {
            Some(pos) => {
                if self.args.len() <= pos + 1 {
                    panic!("No se encontro el valor de seed");
                }

                match self.args[pos + 1].parse::<i32>() {
                    Ok(num) => {
                        self.seed = num;
                        num
                    },
                    Err(_) => {
                        panic!("Error: El valor de -s no es un número entero válido.");
                    }
                }
            },  
            None => {
                panic!("Aviso: semilla no encontrada");
            }
        }
    }

    pub fn get_number_universe(&mut self) -> i32 {
        if self.number_universe < i32::MAX {
            return self.number_universe;
        }

        let pos_s = self.get_position_flag("-N");

        match pos_s {
            Some(pos) => {
                if self.args.len() <= pos + 1 {
                    panic!("No se encontro el valor de N");
                }

                match self.args[pos + 1].parse::<i32>() {
                    Ok(num) => {
                        self.number_universe = num;
                        num
                    },
                    Err(_) => {
                        panic!("Error: El valor de -N no es un número entero válido.");
                    }
                }
            },  
            None => {
                panic!("Aviso: -N no encontrado");
            }
        }
    }

    pub fn get_number_subset(&mut self) -> i32 {
        if self.number_subset < i32::MAX {
            return self.number_subset;
        }

        let pos_s = self.get_position_flag("-S");

        match pos_s {
            Some(pos) => {
                if self.args.len() <= pos + 1 {
                    panic!("No se encontro el valor del número de subconjuntos");
                }

                match self.args[pos + 1].parse::<i32>() {
                    Ok(num) => {
                        self.number_subset = num;
                        num
                    },
                    Err(_) => {
                        panic!("Error: El valor de -S no es un número entero válido.");
                    }
                }
            },  
            None => {
                panic!("Aviso: -S no encontrado");
            }
        }
    }

    pub fn get_help(&self) -> bool {
        self.get_flag("-h") || self.get_flag("--help")
    }

    pub fn print_help(&self) {
        println!("Uso: generate_input [opciones]");
        println!();
        println!("Opciones:");
        println!("  -h --help          Muestra esta ayuda y termina");
        println!("  -N                 Número de elementos del conjunto universo");
        println!("  -s <n>             Inicializa con la semilla n");
        println!("  -S <n>             Genera n subconjuntos aleatorios");
    }

    fn get_position_flag(&self, flag: &str) -> Option<usize> {
        self.args.iter().position(|arg| arg == flag)
    }

    fn get_flag(&self, flag : &'static str) -> bool {
        self.args.iter().any(|arg| arg == flag)
    }
}
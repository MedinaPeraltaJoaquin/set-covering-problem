use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum InputError {
    FileNotFound(String),
    InvalidFormat(String),
    InvalidPath(String),
    NoArgs,
    InvalidArgumentSeed,
    InvalidSeed,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            InputError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            InputError::InvalidPath(msg) => write!(f,"Invalid path: {}",msg),
            InputError::NoArgs => write!(f,"Faltan argumentos"),
            InputError::InvalidArgumentSeed => write!(f,"No se pueden usar ambos argumentos"),
            InputError::InvalidSeed => write!(f,"Seed inválida")
        }
    }
}

impl std::error::Error for InputError {}

pub struct ReadInput {
    pub args : Vec<String>,
    pub seeds : Vec<i32>,
    pub subset : Vec<Vec<String>>
}

impl ReadInput {
    pub fn new(args: Vec<String>) -> Result<Self, InputError> {
        if args.len() == 1 {
            return Err(InputError::NoArgs);
        }
        Ok(ReadInput { args, seeds: vec![], subset : vec![] })
    }

    pub fn read_file_subset(&mut self) -> Result<Vec<Vec<String>>, InputError> {
        if !self.subset.is_empty() {
            return Ok(self.subset.clone());
        }

        let position = if let Some(pos) = self.get_position_flag("-c") {
            if pos + 1 >= self.args.len() {
                return Err(InputError::FileNotFound("No se encontro el valor de path".to_string()));
            }
            pos
        } else {
            return Err(InputError::InvalidPath("No se encontro la bandera".to_string()));
        };

        let next_arg = &self.args[position + 1];
        if next_arg.ends_with(".txt") {
            let content = fs::read_to_string(next_arg)
                        .map_err(|_| InputError::InvalidPath("Error al leer el archivo".to_string()))?;

            let lines: Vec<&str> = content.lines().collect();
            for line in lines {
                let parts_iter: Vec<&str> = line.split(",").collect::<Vec<&str>>();
                let mut unique_parts_ordered: Vec<String> = Vec::new();
                let mut seen: HashSet<&str> = HashSet::new();

                for part in parts_iter {
                    if seen.insert(part) {
                        unique_parts_ordered.push(self.get_element(part).unwrap());
                    }
                }

                
                self.subset.push(unique_parts_ordered);
            }

            
        } else {
            return Err(InputError::InvalidFormat("Debe de ser un archivo .txt".to_string()));
        };

        return Ok(self.subset.clone());
    }


    pub fn get_seed(&mut self) -> Result<Vec<i32>, InputError> {
        if !self.seeds.is_empty() {
            return Ok(self.seeds.clone());
        }

        let pos_s = self.get_position_flag("-s");
        let pos_rs = self.get_position_flag("-rs");

        match (pos_s, pos_rs) {
            (Some(_), Some(_)) => {
                return Err(InputError::InvalidArgumentSeed);
            }
            (Some(pos), None) => {
                if self.args.len() < pos + 1 {
                    return Err(InputError::InvalidFormat("No se encontro el valor de seed".to_string()));
                }

                let start: i32 = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if self.args.len() <= pos + 2 || self.args[pos + 2].starts_with('-') {
                    let seeds = vec![start];
                    self.seeds = seeds.clone();
                    return Ok(seeds);
                }

                let end: i32 = self.args[pos + 2].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if start > end {
                    return Err(InputError::InvalidSeed);
                }

                let seeds: Vec<i32> = (start..=end).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, Some(pos)) => {
                if self.args.len() <= pos + 1 {
                    return Err(InputError::InvalidFormat("No se encontro el valor de seed".to_string()));
                }

                let n: usize = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if n == 0 {
                    return Err(InputError::InvalidSeed);
                }

                use rand::Rng;
                let mut rng = rand::thread_rng();
                let seeds: Vec<i32> = (0..n).map(|_| rng.r#gen()).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, None) => {
                Err(InputError::InvalidSeed)
            }
        }
    }

    pub fn get_verbose(&self) -> bool {
        self.get_flag("-v")
    }

    pub fn get_svg(&self) -> bool {
        self.get_flag("-svg")
    }
    
    pub fn get_help(&self) -> bool {
        self.get_flag("-h") || self.get_flag("--help")
    }

    pub fn print_help(&self) {
        println!("Uso: programa [opciones]");
        println!();
        println!("Opciones:");
        println!("  -h, --help         Muestra esta ayuda y termina");
        println!("  -v                 Activa el modo verbose");
        println!("  -c <path>          Ruta explícita del archivo .txt que representa los subconjuntos del universo");
        println!("  -svg               Activa el modo de salida SVG");
        println!("  -s <I> <F>         Genera semillas en el rango [I, F]");
        println!("  -s <n>             Inicializa con la semilla n");
        println!("  -rs <n>            Genera n semillas aleatorias");
    }

    fn get_flag(&self, flag : &'static str) -> bool {
        self.args.iter().any(|arg| arg == flag)
    }

    fn get_position_flag(&self, flag: &str) -> Option<usize> {
        self.args.iter().position(|arg| arg == flag)
    }

    fn get_element(&self, s: &str) -> Result<String, InputError> {
        let trimmed = s.trim();
        Ok(trimmed.to_string())
    }
}
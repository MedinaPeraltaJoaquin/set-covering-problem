use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug,Clone)]
pub struct Set {
    pub elements : HashMap<String,usize>,
    pub matrix: Vec<bool>,
    pub size_set : usize,
    pub size_subsets : usize,
    pub min_subset : usize,
    pub max_subset : usize
}

impl Set {
    pub fn new(subsets : Vec<Vec<String>>) -> Self{
        let mut elements: HashMap<String, usize> = HashMap::new();

        let mut index: usize = 0;
        for subset in subsets.clone() {
            for element in subset {
                if !elements.contains_key(&element){
                    elements.insert(element, index);
                    index += 1;
                }
            }
        }

        let size: usize = elements.len();

        let lengths = subsets.iter().map(|s| s.len());
        let min_subset = lengths.clone().min().unwrap_or(0);
        let max_subset = lengths.max().unwrap_or(0);

        let mut matrix: Vec<bool> = vec![false; size * size];


        for (i, subset) in subsets.iter().enumerate() {
            for element in subset {
                let index: usize = *elements.get(element)
                    .expect("Elemento del subconjunto no encontrado en el mapa 'elements'");

                matrix[i * size + index] = true;
            }
        }


        Set {
            elements, 
            matrix,
            size_set : size,
            size_subsets : subsets.len(),
            min_subset,
            max_subset
        }
    }

    pub fn get_size_set(&self) -> usize {
        self.size_set
    }

    pub fn get_elements(&self) -> Vec<String> {
        let mut element_list: Vec<String> = self.elements
            .iter()
            .map(|(name, _)| format!("{}", name))
            .collect();

       element_list.sort();
       element_list
    }

    pub fn get_size_subsets(&self) -> usize {
        self.size_subsets
    }


    pub fn get_max_subset(&self) -> usize {
        self.max_subset
    }

    pub fn count_disjoint_subsets(&self, subsets : &Vec<usize>) -> usize {
        if subsets.len() < 2 {
            return 0;
        }

        let mut subconjuntos_con_disjuntos: HashSet<usize> = HashSet::new();

        let mut subset_elements: HashMap<usize, HashSet<usize>> = HashMap::new();
        for &index in subsets.iter() {
            if let Ok(elements) = self.get_elements_in_subset(index) {
                subset_elements.insert(index, elements.into_iter().collect());
            }
        }

        println!("Inicia algoritmo");
        for i in 0..subsets.len() {
            let mut is_fully_disjoint = true;

            let index_i = subsets[i];
            let set_i = subset_elements.get(&index_i).expect("Subconjunto I no encontrado");

            for j in 0..subsets.len() {
                if i == j {
                    continue;
                }

                let index_j = subsets[j];
                let set_j = subset_elements.get(&index_j).expect("Subconjunto J no encontrado");

                let is_disjoint = set_i.is_disjoint(set_j);
                if !is_disjoint {
                    is_fully_disjoint = false;
                    break;
                }
            }

            if is_fully_disjoint {
                subconjuntos_con_disjuntos.insert(index_i);
            }
        }

        subconjuntos_con_disjuntos.len()
    }


    pub fn union_subset(&self, subsets : &Vec<usize>) -> Result<Vec<String>,String> {
        let all_element_under_limit = subsets.iter().all(|&element| {
            element < self.size_subsets
        });

        if !all_element_under_limit {
            let element_fail = subsets.iter()
                            .find(|&element| *element >= self.size_subsets)
                            .unwrap();
            return Err(format!(
                "Error de validación: El elemento {} está fuera del limite {}",
                element_fail,self.size_subsets
            ))
        }  

        let mut element_names: HashMap<usize, String> = HashMap::new();
        for (name, index) in self.elements.clone() {
            element_names.insert(index, name);
        }

        let mut union_subsets : HashSet<String> = HashSet::new();

        for subset_index in subsets {
            for element_index in 0..self.size_set{
                let matrix_index = subset_index * self.size_set + element_index;
                
                if self.matrix[matrix_index] {
                    if let Some(element_name) = element_names.get(&element_index){
                        union_subsets.insert(element_name.clone());
                    }
                }

            }
        }

        let elements_set: Vec<String> = union_subsets.into_iter().collect();
        Ok(elements_set)
    }

    pub fn get_elements_in_subset(&self, subset_index: usize) -> Result<Vec<usize>, String> {
        if subset_index >= self.size_subsets {
            return Err(format!("Error de validación: El subconjunto {} está fuera del límite {}", subset_index, self.size_subsets));
        }
        
        let size = self.size_set;
        let mut covered_element_indices: Vec<usize> = Vec::new();

        for element_index in 0..size {
            let matrix_index = subset_index * size + element_index;
            
            if self.matrix[matrix_index] {
                covered_element_indices.push(element_index);
            }
        }
        
        Ok(covered_element_indices)
    }

    pub fn is_cover_valid(&self, subsets: &Vec<usize>) -> bool{
        match self.union_subset(&subsets) {
            Ok(union) => union.len() == self.size_set,
            Err(_) => false,
        } 
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut element_list: Vec<String> = self.elements
            .iter()
            .map(|(name, index)| format!("{}: {}", name, index))
            .collect();
        element_list.sort();

        write!(f, "
Estructura del Conjunto (Set)
----------------------------------
| Conteo Total de Elementos (|X|): {}
| Subconjuntos Disponibles (|S|): {}
| Tamaño Min/Max de Subconjunto: {} / {}
----------------------------------
| Mapeo de Elementos (Nombre: Índice): 
|   {}
----------------------------------
",
            self.size_set,
            self.size_subsets,
            self.min_subset,
            self.max_subset,
            element_list.join("\n|   ") // Une los elementos con saltos de línea y formato
        )
    }
}
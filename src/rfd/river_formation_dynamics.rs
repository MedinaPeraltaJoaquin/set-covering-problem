use super::super::entity::set::Set;
use super::super::entity::subset_cover::SubsetCover;
use core::f64;
use std::collections::HashSet;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug,Clone)]
pub struct RiverFormationDynamics {
    altitude : Vec<f64>,
    num_drops : usize,
    max_iterations: usize,
    erosion_rate_k : f64,
    sedimentation_rate : f64,
    random : StdRng,
    convergence_curve : Vec<f64>
}

impl RiverFormationDynamics {
    pub fn new(num_drops: usize, 
        max_iterations: usize, 
        erosion_rate_k: f64, 
        sedimentation_rate: f64, 
        initial_altitude: f64, 
        num_subsets: usize,
        seed : u64
    ) -> Self {
        Self {
            altitude: vec![initial_altitude; num_subsets],
            num_drops,
            max_iterations,
            erosion_rate_k,
            sedimentation_rate,
            random : StdRng::seed_from_u64(seed),
            convergence_curve : vec![-1.0;max_iterations]
        }
    }

    pub fn get_convergence_curve(&self) -> Vec<f64> {
        self.convergence_curve.clone()
    }
    pub fn solve(&mut self, set_problem: &Set, verbose_mode : bool) -> SubsetCover {
        let mut best_cover = SubsetCover::new(set_problem);
        let size_elements = set_problem.get_size_set();

        for i in 0..self.max_iterations {
            if verbose_mode {
                println!("B: {}",best_cover.cost);
            }
            self.convergence_curve.insert(i, best_cover.cost);

            for _ in 0..self.num_drops {

                let mut current_cover = SubsetCover::new(set_problem);
                let mut uncovered_elements : HashSet<usize> = (0..size_elements).collect();

                while !uncovered_elements.is_empty() {
                    let mut candidates : Vec<(usize, f64)> = Vec::new();
                    let mut total_probability = 0.0;

                    for subset_index in 0..set_problem.size_subsets {
                        if !current_cover.elements.contains(&subset_index){
                            let elements_in_subset: Vec<usize> = 
                                set_problem.get_elements_in_subset(subset_index)
                                    .expect("Error al obtener elementos del subconjunto");

                            let gain : f64 = elements_in_subset.iter()
                                .filter(|&e| uncovered_elements.contains(e))
                                .count() as f64;

                            if gain > 0.0 {
                                let probabilty = gain /self.altitude[subset_index];
                                
                                candidates.push((subset_index, probabilty));
                                total_probability += probabilty;
                            }
                        } 
                    }

                    if total_probability <=  0.0 {
                         break;
                    }

                    let mut r = self.random.gen_range(0.0..total_probability);
                    let mut chosen_subset_index = candidates[0].0;

                    for (index, probabilty) in candidates {
                        if r <= probabilty {
                            chosen_subset_index = index;
                            break;
                        } 

                        r -= probabilty;
                    }

                    current_cover.add_subset(chosen_subset_index, set_problem);

                    let elements_newly_covered = set_problem.get_elements_in_subset(chosen_subset_index)
                            .expect("Error al obtener elementos del subconjunto elegido");
                        
                    for element_index in elements_newly_covered {
                        uncovered_elements.remove(&element_index);
                    }
                }

                if !uncovered_elements.is_empty() {
                    let penalization_factor = (uncovered_elements.len() as f64 / size_elements as f64) * self.sedimentation_rate;

                    for subsets_index in &current_cover.elements {
                        self.altitude[*subsets_index] += penalization_factor;
                    }

                    continue;
                }

                let cover_cost = current_cover.cost;

                if cover_cost < best_cover.cost {
                    best_cover.apply_neighbour(&current_cover);
                }

                let erosion_amount = self.erosion_rate_k / cover_cost;

                for subset_index in &current_cover.elements {
                    self.altitude[*subset_index] -= erosion_amount;
                    if self.altitude[*subset_index] < 1.0 {
                        self.altitude[*subset_index] = 1.0;
                    }
                }

            }
        }

        self.convergence_curve.remove(0);
        best_cover
    }
}
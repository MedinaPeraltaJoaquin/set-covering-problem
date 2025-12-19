use super::set::Set;

#[derive(Debug,Clone)]

pub struct SubsetCover {
    pub elements : Vec<usize>,
    pub cost : f64,
    pub size : usize
}

impl SubsetCover {
    pub fn new(set : &Set) -> Self{
        SubsetCover { 
            elements: vec![], 
            cost: SubsetCover::calculate_cost(&vec![],set), 
            size : 0
        }
    }

    pub fn add_subset(&mut self, subset_index: usize, set: &Set) -> f64 {
        if !self.elements.contains(&subset_index) {
            self.elements.push(subset_index);
            self.elements.sort_unstable();
            self.size = self.elements.len();
            self.cost = SubsetCover::calculate_cost(&self.elements, set); 
        }

        self.cost
    }

    pub fn get_cost(&mut self, set : &Set) -> f64 {
        SubsetCover::calculate_cost(&self.elements, set)
    }

    pub fn apply_neighbour(&mut self, neighbour : &SubsetCover) {
        self.elements = neighbour.elements.clone();
        self.cost = neighbour.cost;
        self.size = neighbour.size;
    }

    pub fn calculate_cost(subsets : &Vec<usize>, set : &Set) -> f64{
        let union_subsets = match set.union_subset(subsets) {
            Ok(e) => e,
            Err(e) => panic!("{}",e)
        };

        let size_union_subsets = union_subsets.len() as f64;
        let size_subsets = subsets.len() as f64;
        let size_elements = set.get_size_set() as f64;
        let max_subset = set.get_max_subset() as f64;

        //|C| + (|X| - |Union(C)|)max(S)|S| / |S|
        let normalize: f64 = set.get_size_subsets() as f64;
        let raw_cost: f64 = size_subsets + (size_elements - size_union_subsets) * max_subset * normalize;
        raw_cost / normalize
    }
}
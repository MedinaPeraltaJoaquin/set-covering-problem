use super::super::entity::set::Set;
use super::super::entity::subset_cover::SubsetCover;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub struct WriteReport{
    subset_cover : SubsetCover, 
    time : String
}

impl WriteReport {
    pub fn new(subset_cover : SubsetCover, time : String) -> Self {
        WriteReport{
            subset_cover,
            time
        }
    }

    pub fn save_result_subsets(&self, filename : &str, folder : &str, set : &Set, seed : i32) -> io::Result<()> {
        let dir_path = Path::new(folder);
        let file_name = format!("{}.{}.{}.txt", filename,self.time,seed);
        let full_path = dir_path.join(file_name);
        
        fs::create_dir_all(dir_path)?;
        
        let mut output_content = String::new();
        let subsets = self.subset_cover.elements.clone();
        for subset in subsets {
            let line = set.get_elements_in_subset_string(subset)
                .unwrap().join(",");
            
            output_content.push_str(&line);
            output_content.push('\n');
        }

        let mut file = File::create(&full_path)?; 

        file.write_all(output_content.as_bytes())?;

        println!("Datos guardados exitosamente en '{}'", full_path.display());
        Ok(())
    }
}
use std::collections::HashMap;

pub struct School {
    grades : HashMap<u32, Vec<String>>,
} 

impl School {
    pub fn new() -> School {
        School { grades : HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student_name : &str) {
        let entry = self.grades.entry(grade).or_insert(Vec::new());
        let insertion_index = 
            match (*entry).binary_search(&(student_name.to_string())) {
                Ok(x) => x,
                Err(x) => x
            };
        
        (*entry).insert(insertion_index, student_name.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.grades.keys().map(|k| *k).collect::<Vec<u32>>();
        result.sort();
        result
    }

    pub fn grade(&self, grade : u32) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
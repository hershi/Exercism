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

        // To avoid running a full sort every time, we do a sorted insertion.
        // Calling binary_search_by will return the correct insertion index,
        // both in the case when a student with the given name is already in the
        // class, and in the case when such a student is not in the class.
        // We use binary_search_by to allow us to convert the String references
        // to &str for comparison, because there's no 'cmp' defined in the standard
        // library where the LHS is a &String and RHS is a &str.
        let insertion_index = 
            match (*entry).binary_search_by(|s| (s.as_str()).cmp(student_name)) {
                Ok(x) => x,
                Err(x) => x
            };
        
        (*entry).insert(insertion_index, student_name.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.grades.keys().map(|k| *k).collect::<Vec<u32>>();

        // Since keys() returns the keys in arbitrary order, we need to sort them
        // before returning them.
        result.sort();
        result
    }

    pub fn grade(&self, grade : u32) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
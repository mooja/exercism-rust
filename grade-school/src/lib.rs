#[derive(Clone)]
struct Student {
    pub name: String,
    pub grade: u32
}

pub struct School {
    students: Vec<Student>
}

impl School {
    pub fn new() -> School {
        School {
            students: vec![]
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push(Student { name: student.to_string(), grade });
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut rv: Vec<u32> = self.students.iter()
            .map(|s: &Student| s.grade)
            .collect();
        rv.sort();
        rv.dedup();
        rv
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut rv: Vec<String> = self.students.iter()
            .filter(|&s| s.grade == grade)
            .map(|s| s.name.clone())
            .collect();
        rv.sort();
        match rv.len() {
            0 => None,
            _ => Some(rv)
        }
    }
}

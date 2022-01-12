#[derive(Debug)]
pub struct CustomSet<T> {
    pub v: Vec<T>
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut v = vec![];
        for item in _input.into_iter() {
            if !v.contains(item) {
                v.push(item.clone());
            }
        }

        CustomSet {
            v: v
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.v.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.v.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.v.len() > _other.v.len() {
            return false;
        } 

        for item in self.v.iter() {
            if !_other.contains(item) {
                return false;
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        self.v.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        for item in &self.v {
            if _other.contains(item) {
                return false;
            }
        }
        true
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        let mut int_vec = vec![];
        for item in &self.v {
            if _other.contains(item) {
                int_vec.push(item.clone());
            }
        }

        CustomSet {
            v: int_vec
        }        
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let mut diff_vec = vec![];
        for item in &self.v {
            if !_other.contains(item) {
                diff_vec.push(item.clone());
            }
        }

        CustomSet {
            v: diff_vec
        }
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut union_vec = self.v.clone();
        for item in &_other.v {
            if !union_vec.contains(item) {
                union_vec.push(item.clone());
            }
        }

        CustomSet {
            v: union_vec
        }
    }
}

impl <T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.v.len() != other.v.len() {
            return false;
        }

        for item in &self.v {
            if !other.v.contains(item) {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        if self.v.len() != other.v.len() {
            return true;
        }

        for item in &self.v {
            if other.v.contains(item) {
                return false;
            }
        }

        true
    }
}
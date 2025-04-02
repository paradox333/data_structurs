pub struct Array<T> {
    value: Vec<T>,
}

impl<T> Array<T> {
    pub fn new(value: Vec<T>) -> Self {
        Self { value }
    }

    pub fn insert(&mut self, item: T) {
        self.value.push(item)
    }

    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        self.value.get_mut(index)
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.value.len() {
            self.value.remove(index);
        }else{
            println!("Index out of range")
        }
        
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }
}

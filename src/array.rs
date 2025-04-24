// Definición de una estructura genérica Array que envuelve un Vec<T>
pub struct Array<T> {
    value: Vec<T>, // Almacena los datos internamente usando un vector de tipo genérico T
}

// Implementación de métodos para Array<T>
impl<T> Array<T> {
    
    // Crea una nueva instancia de Array a partir de un Vec<T> existente
    pub fn new(value: Vec<T>) -> Self {
        Self { value }
    }

    // Inserta un nuevo elemento al final del array
    pub fn insert(&mut self, item: T) {
        self.value.push(item)
    }

    // Obtiene una referencia mutable al elemento en el índice especificado (si existe)
    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        self.value.get_mut(index)
    }

    // Elimina el elemento en el índice especificado, si está dentro del rango
    pub fn delete(&mut self, index: usize) {
        if index < self.value.len() {
            self.value.remove(index);
        } else {
            // Muestra un mensaje si el índice está fuera de rango
            println!("Index out of range")
        }
    }

    // Retorna la cantidad de elementos actualmente almacenados
    pub fn size(&self) -> usize {
        self.value.len()
    }
}


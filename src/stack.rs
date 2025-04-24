
// Definición de la estructura de un nodo en la pila, que tiene un valor y una referencia al siguiente nodo.
pub struct Node {
    value: usize, // El valor almacenado en el nodo.
    next_node: Option<Box<Node>>, // Enlace al siguiente nodo de la pila (si existe).
}

impl Node {

    // Constructor para crear un nuevo nodo, con un valor y un siguiente nodo.
    pub fn new(value: usize, next_node: Option<Box<Node>>) -> Self {
        Self { value, next_node }
    }
}

// Definición de la estructura de la pila (Stack), que solo tiene la referencia al nodo superior.
pub struct Stack {
    top: Option<Box<Node>>, // El nodo superior de la pila (es un `Box` para tener propiedad y poder moverlo).
}

impl Stack {

    // Constructor para crear una nueva pila vacía.
    pub fn new() -> Self {
        Self { top: None } // Inicializa la pila sin nodos.
    }

    // Método para agregar un nuevo valor en la pila (empujar un valor).
    pub fn push(&mut self, value: usize) {
        // Toma el nodo actual en la parte superior de la pila.
        let current_top = self.top.take();
        // Crea un nuevo nodo con el valor a agregar y el nodo actual como siguiente.
        let new_top = Box::new(Node::new(value, current_top));
        // Actualiza la parte superior de la pila al nuevo nodo.
        self.top = Some(new_top);
    }

    // Método para quitar el nodo superior de la pila (sacar un valor).
    pub fn pop(&mut self) -> Option<Box<Node>> {
        // Si la pila no está vacía, toma el nodo superior, actualiza la pila y devuelve el nodo.
        let current_top = match self.top.take() {
            Some(mut current_node) => {
                self.top = current_node.next_node.take(); // Actualiza el nodo superior a la siguiente parte de la pila.
                Some(current_node) // Devuelve el nodo que fue removido.
            },
            None => None // Si la pila está vacía, devuelve `None`.
        };

        current_top
    }

    // Método para obtener el valor del nodo superior sin removerlo de la pila.
    pub fn top(&self) -> Option<usize> {
        // Si la pila no está vacía, devuelve el valor del nodo superior.
        match &self.top {
            Some(node) => Some(node.value), // Devuelve el valor del nodo superior.
            None => None // Si la pila está vacía, devuelve `None`.
        }
    }

    // Método para verificar si la pila está vacía.
    pub fn is_empty(&self) -> bool {
        // Si la pila tiene un nodo superior, no está vacía.
        match &self.top {
            Some(_node) => false, // Si hay un nodo superior, no está vacía.
            None => true, // Si no hay nodo, la pila está vacía.
        }
    }
}

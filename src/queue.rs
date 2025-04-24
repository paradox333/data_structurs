use std::cell::RefCell;
use std::rc::Rc;

// Nodo de la cola que contiene un valor y un enlace al siguiente nodo
pub struct Node {
    value: usize,                        // El valor del nodo
    next_node: Option<Rc<RefCell<Node>>>  // Enlace al siguiente nodo (opcional y protegido por RefCell y Rc)
}

impl Node {
    // Constructor para crear un nuevo nodo
    pub fn new(value: usize, next_node: Option<Rc<RefCell<Node>>>) -> Self {
        Self { value, next_node }
    }
}

// Cola que contiene los nodos con referencias de tipo Rc y RefCell para el acceso compartido y mutable
pub struct Queue {
    front: Option<Rc<RefCell<Node>>>,   // Nodo al frente de la cola
    back: Option<Rc<RefCell<Node>>>,    // Nodo al final de la cola
}

impl Queue {
    // Constructor para crear una cola vacía
    pub fn new() -> Self {
        Self { front: None, back: None }
    }

    // Método para agregar un valor al final de la cola
    pub fn enqueue(&mut self, value: usize) {
        let new_node = Rc::new(RefCell::new(Node::new(value, None)));  // Creamos un nuevo nodo

        match self.back.take() {
            Some(old_back) => {
                // Si ya existe un nodo al final, actualizamos su siguiente nodo
                old_back.borrow_mut().next_node = Some(new_node.clone());
                self.back = Some(new_node);
            },
            None => {
                // Si la cola está vacía, el nuevo nodo es tanto el frente como el final
                self.front = Some(new_node.clone());
                self.back = Some(new_node);
            }
        }
    }

    // Método para eliminar un nodo del frente de la cola
    pub fn dequeue(&mut self) -> Option<usize> {
        match &self.front.take() {
            Some(node) => {
                // Tomamos el valor del nodo y actualizamos el frente de la cola
                let next = node.borrow().next_node.clone();
                self.front = next;
                Some(node.borrow().value)
            },
            None => {
                println!("end");
                None  // Si la cola está vacía, devolvemos None
            }
        }
    }

    // Método para obtener el valor del nodo al frente de la cola sin eliminarlo
    pub fn peek(&self) -> usize {
        match &self.front {
            Some(node) => node.borrow().value,  // Retorna el valor del nodo en el frente
            None => 0  // Si la cola está vacía, retorna 0
        }
    }

    // Método para verificar si la cola está vacía
    pub fn is_empty(&self) -> bool {
        self.front.is_none()  // Si no hay nodo en el frente, la cola está vacía
    }

    // Método para obtener el tamaño de la cola (número de elementos)
    pub fn len(&self) -> usize {
        Self::len_recursive(self.front.clone())  // Llamada recursiva para contar los elementos
    }

    // Función recursiva para contar el número de nodos en la cola
    fn len_recursive(current_node: Option<Rc<RefCell<Node>>>) -> usize {
        match current_node {
            Some(node_rc) => {
                let next_node_link = node_rc.borrow().next_node.clone();
                1 + Self::len_recursive(next_node_link)  // Contamos el nodo y seguimos con el siguiente
            },
            None => 0  // Cuando llegamos al final de la cola, retornamos 0
        }
    }
}


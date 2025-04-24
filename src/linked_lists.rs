
// Representa un nodo de la lista enlazada
pub struct Node {
    value: usize,                     // El valor que almacena el nodo
    next_node: Option<Box<Node>>      // Apunta al siguiente nodo en la lista (si existe)
}

impl Node {
    // Constructor para crear un nuevo nodo con valor y el siguiente nodo
    pub fn new(value: usize, next_node: Option<Box<Node>>) -> Self {
        Self { value, next_node }
    }
}

// Representa una lista enlazada
pub struct LinkedList {
    head: Option<Box<Node>>,          // El primer nodo de la lista (head)
}

impl LinkedList {

    // Este método permite crear una lista vacía
    pub fn new() -> Self {
        Self { head: None }
    }

    // Este método permite agregar un valor a la lista enlazando los nodos
    pub fn push(&mut self, value: usize) {
        // Creamos un nuevo nodo cuyo siguiente nodo será el actual `head`
        let node = Box::new(Node::new(value, self.head.take()));
        // El `head` de la lista será el nuevo nodo
        self.head = Some(node);
    }

    // Este método permite eliminar el `head` actual y lo reemplaza por el siguiente nodo
    pub fn pop(&mut self) -> Option<usize> {
        // Tomamos el nodo `head` y actualizamos el `head` al siguiente nodo
        self.head.take().map(|node| {
            self.head = node.next_node;
            node.value  // Devolvemos el valor del nodo eliminado
        })
    }

    // Este método retorna el valor del `head` actual sin modificar la lista
    pub fn peek(&self) -> Option<usize> {
        // Si la lista no está vacía, devolvemos el valor del `head`
        self.head.as_ref().map(|node| {
            node.value
        })
    }

    // Este método retorna si la lista está vacía
    pub fn is_empty(&self) -> bool {
        // Si `head` es `None`, la lista está vacía
        self.head.is_none()
    }
}



pub struct Node {
    value: usize,
    next_node: Option<Box<Node>>
}

impl Node {

    pub fn new( value: usize, next_node: Option<Box<Node>> ) -> Self {
        Self { value, next_node }
    }


}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {

    //Este método permite crear una lista vacia
    pub fn new() -> Self {
        Self { head: None }
    }
    //Este método permite agregar un valor a la lista enlazando los nodos.
    pub fn push(&mut self, value: usize){
        let node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(node);
    }
    //Este método permite eliminar el head actual y lo reemplaza por el sigiente.
    pub fn pop(&mut self) -> Option<usize>{
        self.head.take().map(|node| {
            self.head = node.next_node;
            node.value
        })
    }
    //Este método retorna el valor del head actual sin modificar el valor
    pub fn peek(&self) -> Option<usize>{
        self.head.as_ref().map(|node| {
            node.value
        })
    }
    //Este método retorna si la lista está o no vacía
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

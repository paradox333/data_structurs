// Representa un nodo de un grafo
pub struct Node {
    pub id: usize,                     // Identificador único del nodo
    pub adjacency_list: Vec<usize>,   // Lista de nodos adyacentes (por ID)
}

impl Node {
    // Constructor para crear un nuevo nodo con un ID y una lista de adyacencia inicial
    pub fn new(id: usize, adjacency_list: Vec<usize>) -> Self {
        Self { id, adjacency_list }
    }

    // Inserta el ID de un nodo vecino en la lista de adyacencia
    pub fn insert(&mut self, neighbor_id: usize) {
        self.adjacency_list.push(neighbor_id)
    }
}


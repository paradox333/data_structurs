pub struct Node {
    pub id: usize,
    pub adjacency_list: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, adjacency_list: Vec<usize>) -> Self {
        Self { id, adjacency_list }
    }

    pub fn insert(&mut self, neighbor_id: usize) {
        self.adjacency_list.push(neighbor_id)
    }

}

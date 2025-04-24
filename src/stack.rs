pub struct Node {
    value: usize,
    next_node: Option<Box<Node>>
}

impl Node {

    pub fn new( value: usize, next_node: Option<Box<Node>> ) -> Self {
        Self { value, next_node }
    }
}

pub struct Stack {
    top: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Self {
        Self { top: None }
    }
    pub fn push(&mut self, value: usize){
        let current_top = self.top.take();
        let new_top = Box::new(Node::new(value, current_top));
        self.top = Some(new_top);
    }
    pub fn pop(&mut self) -> Option<Box<Node>> {

        let current_top = match self.top.take() {
            Some(mut current_node) => {
                self.top = current_node.next_node.take();
                Some(current_node)
            },
            None => None
        };

        current_top
    }
    pub fn top(&self) -> Option<usize> {
        match &self.top {
            Some(node) => {
                return Some(node.value)
            },
            None => None
        }
    }
    pub fn is_empty(&self) -> bool {
        return match &self.top {
            Some (_node) => false,
            None => true
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    value: usize,
    next_node: Option<Rc<RefCell<Node>>>
}

impl Node {

    pub fn new( value: usize, next_node: Option<Rc<RefCell<Node>>> ) -> Self {
        Self { value, next_node }
    }
}

pub struct Queue {
    front: Option<Rc<RefCell<Node>>>,
    back: Option<Rc<RefCell<Node>>>
}

impl Queue {
    pub fn new() -> Self {
        Self{ front: None, back: None }
    }

    pub fn enqueue(&mut self, value: usize){
        let new_node = Rc::new(RefCell::new(Node::new(value, None)));

        match self.back.take() {
            Some(old_back) => {
                old_back.borrow_mut().next_node = Some(new_node.clone());
                self.back = Some(new_node);
            },
            None => {
                self.front = Some(new_node.clone()); // <-- Actualiza front
                self.back = Some(new_node);
            }
        }
    }
    pub fn dequeue(&mut self) -> Option<usize> {
        match &self.front.take() {
            Some(node) => {
                let next = node.borrow().next_node.clone();
                self.front = next;
                Some(node.borrow().value)
            },
            None => {
                println!("end");
                None
            }
        }
    }
    pub fn peek(&self) -> usize {
        return match &self.front {
            Some(node) => {
                node.borrow().value
            },
            None => 0
        }
    }
    pub fn is_empty(&self) -> bool {
        return match &self.front {
            Some(_node) => false,
            None => true
        }
    }
    pub fn len(&self) -> usize {
        return Self::len_recursive(self.front.clone());
    }

    fn len_recursive(current_node: Option<Rc<RefCell<Node>>>) -> usize {
        match current_node {
            Some(node_rc) => {
                let next_node_link = node_rc.borrow().next_node.clone();
                1 + Self::len_recursive(next_node_link)
            },
            None => 0
        }
    }
}

use std::sync::{Arc, Mutex};

pub struct Node {
    value: usize,
    node_left: Option<Arc<Mutex<Node>>>,
    node_right: Option<Arc<Mutex<Node>>>,
}

impl Node {
    pub fn new(value: usize) -> Self {
        Self {
            value, 
            node_left: None, 
            node_right: None
        }
    }
}

pub struct BinaryTree {
    root: Option<Arc<Mutex<Node>>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: usize){
        let new_node = Arc::new(Mutex::new(Node::new(value)));

        match &self.root {
            Some(root_arc) => {
                let mut root = root_arc.lock().unwrap();
                Self::insert_recursive(&mut root, new_node);
            }
            None => {
                self.root = Some(new_node);
            }
        }
        
    }

    fn insert_recursive(current_node: &mut Node, new_node: Arc<Mutex<Node>>){
        let new_value = new_node.lock().unwrap().value;

        if new_value < current_node.value {
            match &mut current_node.node_left {
                Some(left_node) => {
                    let mut left = left_node.lock().unwrap();
                    Self::insert_recursive(&mut left, new_node);
                }
                None => {
                    current_node.node_left = Some(new_node);
                }
            } 
        } else {
            match &mut current_node.node_right {
                Some(right_node) => {
                    let mut right = right_node.lock().unwrap();
                    Self::insert_recursive(&mut right, new_node);
                }
                None => {
                    current_node.node_right = Some(new_node);
                }
            }
            
        }
    }

    pub fn contains(&self, value: usize) -> bool {

        match &self.root {
            Some(node_root) => {
                let node = node_root.lock().unwrap();
                return Self::contains_recursive(&*node, value);
            }
            None => false
        }
    }

    fn contains_recursive(current_node: &Node, value: usize) -> bool{
        if current_node.value == value {
            true
        } else if current_node.value < value {
            match &current_node.node_right {
                Some(right_node) => {
                    let right = right_node.lock().unwrap();
                    return Self::contains_recursive(&*right, value);
                }
                None => false
            }

        } else {
            match &current_node.node_left {
                Some(left_node) => {
                    let left = left_node.lock().unwrap();
                    return Self::contains_recursive(&*left, value);
                }
                None => false
            }
        }
    }


    
    pub fn remove(&mut self, value: usize) -> bool {
        Self::remove_recursive(&mut self.root, value)
    }

    
   
    fn remove_recursive(current: &mut Option<Arc<Mutex<Node>>>, value: usize) -> bool {
    if let Some(node_arc) = current.clone() {
        let mut remove_self = false;
        let mut replace_with = None;

        {
            let mut node = node_arc.lock().unwrap();

            if value < node.value {
                return Self::remove_recursive(&mut node.node_left, value);
            } else if value > node.value {
                return Self::remove_recursive(&mut node.node_right, value);
            }

            // Encontramos el nodo
            if node.node_left.is_none() && node.node_right.is_none() {
                remove_self = true;
            } else if node.node_left.is_none() {
                replace_with = node.node_right.clone();
            } else if node.node_right.is_none() {
                replace_with = node.node_left.clone();
            } else {
                // Caso: tiene dos hijos
                let mut successor = node.node_right.clone();
                let successor_value;
                loop {
                    let left = {
                        let suc = successor.as_ref().unwrap().lock().unwrap();
                        suc.node_left.clone()
                    };

                    if let Some(left_node) = left {
                        successor = Some(left_node);
                    } else {
                        break;
                    }
                }

                successor_value = successor.as_ref().unwrap().lock().unwrap().value;
                node.value = successor_value;
                return Self::remove_recursive(&mut node.node_right, successor_value);
            }
        }

        // Fuera del scope: ahora sí puedes modificar `*current`
        if remove_self {
            *current = None;
        } else if let Some(new_node) = replace_with {
            *current = Some(new_node);
        }

        return true;
    }

    false
}

    
    pub fn in_order(&self) -> Vec<usize> {
        let mut list_result = Vec::new();

        if let Some(root_arc) = &self.root {
            let root = root_arc.lock().unwrap();
            Self::in_order_recursive(&root, &mut list_result);
        }

        list_result
    }


    fn in_order_recursive(current_node: &Node, list_result: &mut Vec<usize>){

        if !current_node.node_left.is_none() {
            if let Some(node_left) = &current_node.node_left {
                let left = node_left.lock().unwrap();
                Self::in_order_recursive(&left, list_result);
            }
        }

        list_result.push(current_node.value);

        if !current_node.node_right.is_none() {
            if let Some(node_right) = &current_node.node_right {
                let right = node_right.lock().unwrap();
                Self::in_order_recursive(&right, list_result);
            }
        }
    }


    pub fn height(&self) -> u64{
        match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::height_recursive(&*node);
            },
            None => 0
        }
    }

    fn height_recursive(node: &Node) -> u64 {

        let left_height = match &node.node_left {
            Some(left_arc) => {
                let left = left_arc.lock().unwrap();
                Self::height_recursive(&left)
            },
            None => 0,
        };
        let right_height = match &node.node_right {
            Some(right_arc) => {
                let right = right_arc.lock().unwrap();
                Self::height_recursive(&right)
            },
            None => 0,
        };
        
        1 + left_height.max(right_height)
    }

    pub fn is_empty(&self) -> bool{
        return match &self.root {
            Some(_node) => false,
            None => true
        };

    }

    pub fn min(&self) -> usize {
         match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::min_recursive(&*node);
            },
            None => 0
        }
    }

    fn min_recursive(current_node: &Node) -> usize {
        if let Some(ref left_arc) = current_node.node_left {
            let left_node = left_arc.lock().unwrap();
            return Self::min_recursive(&left_node);
        }
        current_node.value
    }

    pub fn max(&self) -> usize{
        return match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::max_recursive(&*node);
            },
            None => 0
        }
    }

    fn max_recursive(current_node: &Node) -> usize {
        if let Some(ref right_arc) = current_node.node_right {
            let right_node = right_arc.lock().unwrap();
            return Self::max_recursive(&right_node);
        }
        current_node.value
    }

    pub fn size(&self) -> usize {
        return match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::size_recursive(&*node) + 1
            },
            None => 0
        }
    }

    fn size_recursive(current_node: &Node) -> usize {
        let left_height = match &current_node.node_left {
            Some(left_arc) => {
                let left = left_arc.lock().unwrap();
                Self::size_recursive(&left) + 1
            },
            None => 0,
        };
        let right_height = match &current_node.node_right {
            Some(right_arc) => {
                let right = right_arc.lock().unwrap();
                Self::size_recursive(&right) + 1
            },
            None => 0,
        };
        left_height + right_height
    }

    pub fn clear(&mut self) {
        self.root = None;
    }
}




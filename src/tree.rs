use std::sync::{Arc, Mutex};

// Nodo que contiene un valor y referencias a los nodos izquierdo y derecho
pub struct Node {
    value: usize,                      // El valor almacenado en el nodo
    node_left: Option<Arc<Mutex<Node>>>,   // Nodo izquierdo (opcional y protegido por un Mutex)
    node_right: Option<Arc<Mutex<Node>>>,  // Nodo derecho (opcional y protegido por un Mutex)
}

impl Node {
    // Constructor para crear un nuevo nodo con un valor dado
    pub fn new(value: usize) -> Self {
        Self {
            value, 
            node_left: None, 
            node_right: None,
        }
    }
}

// Estructura que representa un árbol binario
pub struct BinaryTree {
    root: Option<Arc<Mutex<Node>>>,  // La raíz del árbol, protegida por Mutex
}

impl BinaryTree {
    // Constructor para crear un árbol vacío
    pub fn new() -> Self {
        Self { root: None }
    }

    // Método para insertar un nuevo valor en el árbol
    pub fn insert(&mut self, value: usize){
        let new_node = Arc::new(Mutex::new(Node::new(value)));

        match &self.root {
            Some(root_arc) => {
                let mut root = root_arc.lock().unwrap();
                Self::insert_recursive(&mut root, new_node); // Inserción recursiva
            }
            None => {
                self.root = Some(new_node);  // Si el árbol está vacío, establecemos la raíz
            }
        }
    }

    // Método recursivo para insertar un nuevo nodo en el árbol
    fn insert_recursive(current_node: &mut Node, new_node: Arc<Mutex<Node>>){
        let new_value = new_node.lock().unwrap().value;

        if new_value < current_node.value {
            match &mut current_node.node_left {
                Some(left_node) => {
                    let mut left = left_node.lock().unwrap();
                    Self::insert_recursive(&mut left, new_node); // Continuamos la inserción en el subárbol izquierdo
                }
                None => {
                    current_node.node_left = Some(new_node);  // Insertamos en el subárbol izquierdo
                }
            } 
        } else {
            match &mut current_node.node_right {
                Some(right_node) => {
                    let mut right = right_node.lock().unwrap();
                    Self::insert_recursive(&mut right, new_node); // Continuamos la inserción en el subárbol derecho
                }
                None => {
                    current_node.node_right = Some(new_node); // Insertamos en el subárbol derecho
                }
            }
        }
    }

    // Método para verificar si un valor existe en el árbol
    pub fn contains(&self, value: usize) -> bool {
        match &self.root {
            Some(node_root) => {
                let node = node_root.lock().unwrap();
                return Self::contains_recursive(&*node, value);  // Búsqueda recursiva
            }
            None => false  // Si el árbol está vacío, el valor no está presente
        }
    }

    // Método recursivo para verificar si un valor existe en el subárbol
    fn contains_recursive(current_node: &Node, value: usize) -> bool {
        if current_node.value == value {
            true  // El valor fue encontrado
        } else if current_node.value < value {
            match &current_node.node_right {
                Some(right_node) => {
                    let right = right_node.lock().unwrap();
                    return Self::contains_recursive(&*right, value);  // Búsqueda en el subárbol derecho
                }
                None => false  // Si no hay nodo derecho, el valor no está
            }
        } else {
            match &current_node.node_left {
                Some(left_node) => {
                    let left = left_node.lock().unwrap();
                    return Self::contains_recursive(&*left, value);  // Búsqueda en el subárbol izquierdo
                }
                None => false  // Si no hay nodo izquierdo, el valor no está
            }
        }
    }

    // Método para eliminar un nodo con un valor dado
    pub fn remove(&mut self, value: usize) -> bool {
        Self::remove_recursive(&mut self.root, value)
    }

    // Método recursivo para eliminar un nodo en el árbol
    fn remove_recursive(current: &mut Option<Arc<Mutex<Node>>>, value: usize) -> bool {
        if let Some(node_arc) = current.clone() {
            let mut remove_self = false;
            let mut replace_with = None;

            {
                let mut node = node_arc.lock().unwrap();

                // Si el valor es menor que el nodo actual, vamos a la izquierda
                if value < node.value {
                    return Self::remove_recursive(&mut node.node_left, value);
                } else if value > node.value {
                    return Self::remove_recursive(&mut node.node_right, value);
                }

                // Caso: encontramos el nodo a eliminar
                if node.node_left.is_none() && node.node_right.is_none() {
                    remove_self = true;  // Si el nodo no tiene hijos, lo eliminamos
                } else if node.node_left.is_none() {
                    replace_with = node.node_right.clone();  // Si solo tiene hijo derecho, lo reemplazamos
                } else if node.node_right.is_none() {
                    replace_with = node.node_left.clone();  // Si solo tiene hijo izquierdo, lo reemplazamos
                } else {
                    // Caso: el nodo tiene dos hijos
                    let mut successor = node.node_right.clone();
                    let successor_value;
                    loop {
                        let left = {
                            let suc = successor.as_ref().unwrap().lock().unwrap();
                            suc.node_left.clone()
                        };

                        if let Some(left_node) = left {
                            successor = Some(left_node);  // Buscamos el sucesor más pequeño
                        } else {
                            break;
                        }
                    }

                    successor_value = successor.as_ref().unwrap().lock().unwrap().value;
                    node.value = successor_value;
                    return Self::remove_recursive(&mut node.node_right, successor_value);  // Eliminamos el sucesor
                }
            }

            // Fuera del scope de bloqueo de Mutex: modificamos `*current`
            if remove_self {
                *current = None;  // El nodo no tiene hijos, se elimina
            } else if let Some(new_node) = replace_with {
                *current = Some(new_node);  // Reemplazamos el nodo con su hijo
            }

            return true;  // Nodo eliminado
        }

        false  // El nodo no fue encontrado
    }

    // Método para obtener una lista de los valores en orden (in-order traversal)
    pub fn in_order(&self) -> Vec<usize> {
        let mut list_result = Vec::new();

        if let Some(root_arc) = &self.root {
            let root = root_arc.lock().unwrap();
            Self::in_order_recursive(&root, &mut list_result);  // Llamada recursiva
        }

        list_result  // Retorna la lista de valores
    }

    // Recorrido in-order recursivo para agregar los valores a la lista
    fn in_order_recursive(current_node: &Node, list_result: &mut Vec<usize>) {
        if let Some(ref left_arc) = current_node.node_left {
            let left_node = left_arc.lock().unwrap();
            Self::in_order_recursive(&left_node, list_result);  // Recorrido en el subárbol izquierdo
        }

        list_result.push(current_node.value);  // Agregamos el valor del nodo

        if let Some(ref right_arc) = current_node.node_right {
            let right_node = right_arc.lock().unwrap();
            Self::in_order_recursive(&right_node, list_result);  // Recorrido en el subárbol derecho
        }
    }

    // Método para obtener la altura del árbol
    pub fn height(&self) -> u64 {
        match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::height_recursive(&*node);  // Llamada recursiva para calcular la altura
            },
            None => 0  // Si el árbol está vacío, la altura es 0
        }
    }

    // Función recursiva para calcular la altura de un subárbol
    fn height_recursive(node: &Node) -> u64 {
        let left_height = match &node.node_left {
            Some(left_arc) => {
                let left = left_arc.lock().unwrap();
                Self::height_recursive(&left)  // Altura del subárbol izquierdo
            },
            None => 0,
        };
        let right_height = match &node.node_right {
            Some(right_arc) => {
                let right = right_arc.lock().unwrap();
                Self::height_recursive(&right)  // Altura del subárbol derecho
            },
            None => 0,
        };

        1 + left_height.max(right_height)  // La altura es el máximo de los subárboles más 1
    }

    // Método para verificar si el árbol está vacío
    pub fn is_empty(&self) -> bool {
        match &self.root {
            Some(_) => false,  // Si hay un nodo, el árbol no está vacío
            None => true,  // Si no hay nodos, el árbol está vacío
        }
    }

    // Método para obtener el valor mínimo en el árbol
    pub fn min(&self) -> usize {
        match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::min_recursive(&*node);  // Llamada recursiva para obtener el valor mínimo
            },
            None => 0  // Si el árbol está vacío, retornamos 0
        }
    }

    // Función recursiva para obtener el valor mínimo en el subárbol
    fn min_recursive(current_node: &Node) -> usize {
        if let Some(ref left_arc) = current_node.node_left {
            let left_node = left_arc.lock().unwrap();
            return Self::min_recursive(&left_node);  // Continuamos buscando en el subárbol izquierdo
        }
        current_node.value  // Si no hay nodo izquierdo, este es el mínimo
    }

    // Método para obtener el valor máximo en el árbol
    pub fn max(&self) -> usize {
        return match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::max_recursive(&*node);  // Llamada recursiva para obtener el valor máximo
            },
            None => 0  // Si el árbol está vacío, retornamos 0
        }
    }

    // Función recursiva para obtener el valor máximo en el subárbol
    fn max_recursive(current_node: &Node) -> usize {
        if let Some(ref right_arc) = current_node.node_right {
            let right_node = right_arc.lock().unwrap();
            return Self::max_recursive(&right_node);  // Continuamos buscando en el subárbol derecho
        }
        current_node.value  // Si no hay nodo derecho, este es el máximo
    }

    // Método para obtener el tamaño del árbol (número de nodos)
    pub fn size(&self) -> usize {
        return match &self.root {
            Some(node_arc) => {
                let node = node_arc.lock().unwrap();
                return Self::size_recursive(&*node) + 1  // Llamada recursiva para calcular el tamaño
            },
            None => 0  // Si el árbol está vacío, el tamaño es 0
        }
    }

    // Función recursiva para calcular el tamaño del subárbol
    fn size_recursive(current_node: &Node) -> usize {
        let left_size = match &current_node.node_left {
            Some(left_arc) => {
                let left = left_arc.lock().unwrap();
                Self::size_recursive(&left) + 1  // Tamaño del subárbol izquierdo
            },
            None => 0,
        };
        let right_size = match &current_node.node_right {
            Some(right_arc) => {
                let right = right_arc.lock().unwrap();
                Self::size_recursive(&right) + 1  // Tamaño del subárbol derecho
            },
            None => 0,
        };
        left_size + right_size  // Sumar el tamaño de los dos subárboles
    }

    // Método para limpiar el árbol (eliminar todos los nodos)
    pub fn clear(&mut self) {
        self.root = None;  // Establece la raíz a None, eliminando efectivamente el árbol
    }
}




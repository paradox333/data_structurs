// Declaración de los módulos definidos en otros archivos.
// Cada uno representa una estructura de datos diferente.
mod array;
mod graph;
mod linked_lists;
mod hash;
mod tree;
mod stack;
mod queue;

// Importación de estructuras públicas desde los módulos para su uso en main.
use queue::Queue;
use stack::Stack;
use hash::HashMap;
use tree::BinaryTree;

// Función principal del programa.
// Aquí se llaman funciones de prueba para cada estructura de datos.
fn main() {
    // Prueba de operaciones con arrays (arreglos)
    test_array();
    
    // Prueba de funcionalidades en grafos
    test_graph();
    
    // Prueba de listas enlazadas
    test_linked_list();
    
    // Prueba de operaciones de un mapa hash (HashMap)
    test_hash();
    
    // Prueba de inserción, recorrido y otras funciones de un árbol binario
    test_tree();
    
    // Prueba de operaciones LIFO (Last In First Out) en la pila (stack)
    test_stack();
    
    // Prueba de operaciones FIFO (First In First Out) en la cola (queue)
    test_queue();
}



fn test_queue(){
    // Inicio de la prueba para la estructura de datos Queue (cola)
    println!("---------Test Queue---------");

    // Creación de una nueva instancia de la cola
    println!("---------Creating Queue---------");
    let mut queue = Queue::new();

    // Verifica e imprime la longitud de la cola (debería ser 0 inicialmente)
    println!("Length: {}", queue.len());

    // Verifica si la cola está vacía (true al inicio)
    println!("Is empty? {}", queue.is_empty());

    // Inserta el valor 3 en la cola
    println!("Add 3:");
    queue.enqueue(3);

    // Inserta el valor 2 en la cola
    println!("Add 2:");
    queue.enqueue(2);

    // Elimina y muestra el primer elemento en ser insertado (FIFO), que debería ser 3
    if let Some(value) = queue.dequeue() {
        println!("Value removed: {}", value);
    } else {
        println!("Not found value");
    }

    // Verifica si la cola está vacía luego de hacer un dequeue
    println!("Is empty? {}", queue.is_empty());

    // Muestra el valor del frente de la cola sin eliminarlo (debería ser 2)
    println!("Peek: {}", queue.peek());

    // Muestra la longitud actual de la cola
    println!("Length: {}", queue.len());
}



fn test_stack(){
    // Inicio de la prueba para la estructura de datos Stack (pila)
    println!("---------Test Stack---------");

    // Creación de una nueva instancia de la pila
    let mut stack = Stack::new();

    // Verifica si la pila está vacía al inicio (debería ser true)
    println!("is empty? {}", stack.is_empty());

    // Inserta el valor 3 en la pila
    stack.push(3);

    // Obtiene y muestra el valor actual en la cima de la pila (debería ser 3)
    let top = stack.top().take().unwrap();
    println!("Current top: {}", top);

    // Elimina el valor en la cima de la pila
    stack.pop();

    // Inserta los valores 1 y luego 2 en la pila (LIFO)
    stack.push(1);
    stack.push(2);

    // Obtiene y muestra el valor actual en la cima (debería ser 2)
    let top = stack.top().take().unwrap();
    println!("Current top: {}", top);

    // Verifica si la pila está vacía después de varias operaciones
    println!("is empty? {}", stack.is_empty());

    // Elimina el valor en la cima (2)
    stack.pop();

    // Muestra el nuevo valor en la cima (debería ser 1)
    let top = stack.top().take().unwrap();
    println!("current top: {}", top);

    // Elimina el último elemento (1), la pila queda vacía
    stack.pop();

    // Intenta obtener el top de una pila vacía (debería ser None)
    let top = stack.top().take();
    println!("current top: {:?}", top);
}


fn test_tree(){
    println!("---------Test Tree---------");

    // Lista de números primos para insertar en el árbol binario
    let primes: Vec<usize> = vec![
        19, 107, 41, 29, 109, 11, 67, 
        31, 3, 103, 73, 127, 43, 47, 
        83, 5, 17, 89, 59, 101, 79, 
        113, 71, 23, 97, 61, 13, 7, 
        2, 53, 137, 131, 37
    ];

    // Se crea un nuevo árbol binario vacío
    let mut binary_tree: BinaryTree = tree::BinaryTree::new();

    // Verifica si el árbol está vacío
    println!("is empty? {:?}", binary_tree.is_empty());

    // Imprime la altura actual del árbol (debería ser 0)
    println!("Height: {}", binary_tree.height());

    // Inserta todos los números primos en el árbol
    for number in primes.iter() {
        binary_tree.insert(*number)
    }

    // Muestra la nueva altura del árbol después de insertar los valores
    println!("Height: {}", binary_tree.height());

    // Verifica si el árbol contiene el valor 3 (debería ser true)
    println!("contains value 3? {}", binary_tree.contains(3));

    // Verifica si el árbol contiene el valor 6 (no fue insertado, debería ser false)
    println!("contains value 6? {}", binary_tree.contains(6));

    // Elimina el valor 3 del árbol
    binary_tree.remove(3);

    // Verifica nuevamente si el valor 3 sigue presente (debería ser false)
    println!("contains value 3? {}", binary_tree.contains(3));

    // Obtiene e imprime los valores del árbol en recorrido in-order
    let in_order = binary_tree.in_order();
    println!("in order: {:?}", in_order);

    // Verifica si el árbol está vacío (debería ser false)
    println!("is empty? {:?}", binary_tree.is_empty());

    // Obtiene el valor mínimo del árbol
    println!("min: {}", binary_tree.min());

    // Obtiene el valor máximo del árbol
    println!("max: {}", binary_tree.max());

    // Imprime el tamaño actual del árbol (número de nodos)
    println!("size: {}", binary_tree.size());

    // Limpia el árbol (elimina todos los nodos)
    println!("clear:");
    binary_tree.clear();

    // Imprime el tamaño del árbol después de limpiarlo (debería ser 0)
    println!("size: {}", binary_tree.size());
}



fn test_hash(){
    println!("---------Test Table Hash---------");

    // Se crea una nueva tabla hash personalizada con claves tipo String y valores tipo i32
    let mut hash_map: HashMap<String, i32> = hash::HashMap::new();

    // Inserta un par clave-valor en la tabla: ("key_test_1", 33)
    hash_map.insert("key_test_1".to_string(), 33);

    // Intenta obtener el valor asociado a "key_test_1"
    if let Some(row) = hash_map.get(&"key_test_1".to_string()) {
        // Si se encuentra, imprime el valor almacenado
        println!("Row {}", row.value);
    } else {
        // Si no se encuentra, informa que la clave no existe
        println!("key not found");
    }

    // Elimina la clave "key_test_1" de la tabla
    hash_map.remove(&"key_test_1".to_string());

    // Vuelve a intentar obtener el valor asociado a "key_test_1" después de eliminarlo
    if let Some(row) = hash_map.get(&"key_test_1".to_string()) {
        // Esto no debería imprimirse porque la clave fue eliminada
        println!("Row {}", row.value);
    } else {
        // Confirma que la clave ya no existe
        println!("key not found");
    }
}



fn test_linked_list(){
    println!("---------Test Linked List---------");

    // Crea una nueva lista enlazada vacía
    let mut list = linked_lists::LinkedList::new();

    // Verifica si la lista está vacía
    println!("Is empty? {}", list.is_empty());

    // Inserta el valor 2 al inicio de la lista
    list.push(2);

    // Vuelve a verificar si la lista está vacía (debería ser false ahora)
    println!("Is empty? {}", list.is_empty());

    // Muestra el valor en la cabeza de la lista sin eliminarlo
    println!("Head value: {:?}", list.peek());

    // Elimina y muestra el valor en la cabeza de la lista
    println!("Delete head: {:?}", list.pop());

    // Intenta mostrar el nuevo valor en la cabeza (debería ser None si no hay más elementos)
    println!("Head value: {:?}", list.peek());
}

fn test_graph(){
    println!("---------Test Graph---------");

    // Crea un nodo con id 1 y una lista de adyacencia vacía
    let mut graph = graph::Node::new(1, Vec::new());
    println!("Primer nodo id: {}", graph.id);

    // Crea dos nodos adicionales con id 2 y 3, también con listas de adyacencia vacías
    let mut graph_2 = graph::Node::new(2, Vec::new());
    let mut graph_3 = graph::Node::new(3, Vec::new());

    // Agrega una conexión del nodo 2 al nodo 3
    graph_2.insert(graph_3.id);

    // Agrega una conexión del nodo 3 al nodo 1
    graph_3.insert(graph.id);

    // Agrega conexiones del nodo 1 a los nodos 2 y 3
    graph.insert(graph_2.id);
    graph.insert(graph_3.id);

    // Muestra las listas de adyacencia de cada nodo
    println!("Primer nodo lista de adyacencia: {:?}", graph.adjacency_list);
    println!("Segundo nodo lista de adyacencia: {:?}", graph_2.adjacency_list);
    println!("Tercer nodo lista de adyacencia: {:?}", graph_3.adjacency_list);
}


fn test_array(){
    println!("---------Test Array---------");

    // Crea un nuevo array con los valores iniciales [1, 2, 3]
    let mut arr = array::Array::new(vec![1, 2, 3]);

    // Muestra el tamaño actual del array
    println!("Tamaño del array: {}", arr.size());

    // Elimina el elemento en la posición 0 (el primer elemento)
    arr.delete(0);

    // Muestra el tamaño del array luego de eliminar un elemento
    println!("Tamaño del array: {}", arr.size());

    // Inserta el valor 35 al final del array
    arr.insert(35);

    // Muestra el nuevo tamaño del array
    println!("Tamaño del array: {}", arr.size());

    // Obtiene y muestra el último dato del array
    println!("El ultimo dato es: {:?}", arr.get(arr.size() - 1));
    
    // Si existe un valor al final del array, lo obtiene, le resta 2 y lo muestra
    if let Some(value) = arr.get(arr.size() - 1) {
        let new_value = *value - 2;
        println!("El ultimo dato es: {:?}", new_value);
    } else {
        println!("Not found value");
    }
}


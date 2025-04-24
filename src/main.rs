mod array;
mod graph;
mod linked_lists;
mod hash;
mod tree;
use hash::HashMap;
use tree::BinaryTree;

fn main() {
    //test_array();
    //test_graph();
    //test_linked_list();
    //test_hash();
    test_tree();
}

fn test_tree(){
    let primes: Vec<usize> = vec![
        19, 107, 41, 29, 109, 11, 67, 
        31, 3, 103, 73, 127, 43, 47, 
        83, 5, 17, 89, 59, 101, 79, 
        113, 71, 23, 97, 61, 13, 7, 
        2, 53, 137, 131, 37
    ];
    let mut binary_tree: BinaryTree = tree::BinaryTree::new();
    println!("is empty? {:?}", binary_tree.is_empty());
    println!("Height: {}", binary_tree.height());
    for number in primes.iter() {
        binary_tree.insert(*number)
    }
    println!("Height: {}", binary_tree.height());
    println!("contains value 3? {}", binary_tree.contains(3));
    println!("contains value 6? {}", binary_tree.contains(6));
    binary_tree.remove(3);
    println!("contains value 3? {}", binary_tree.contains(3));
    let in_order = binary_tree.in_order();
    println!("in order: {:?}", in_order);
    println!("is empty? {:?}", binary_tree.is_empty());
    println!("min: {}", binary_tree.min());
    println!("max: {}", binary_tree.max());
    println!("size: {}", binary_tree.size());
    println!("clear:");
    binary_tree.clear();
    println!("size: {}", binary_tree.size());
}

fn test_hash(){
    let mut hash_map: HashMap<String, i32> = hash::HashMap::new();
    hash_map.insert("key_test_1".to_string(), 33);
    if let Some(row) = hash_map.get(&"key_test_1".to_string()) {
        println!("Row {}", row.value);
    } else {
        println!("key not found");
    }
    hash_map.remove(&"key_test_1".to_string());
    if let Some(row) = hash_map.get(&"key_test_1".to_string()) {
        println!("Row {}", row.value);
    } else {
        println!("key not found");
    }

}

fn test_linked_list(){
    let mut list = linked_lists::LinkedList::new();
    println!("Is empty? {}", list.is_empty());
    list.push(2);
    println!("Is empty? {}", list.is_empty());
    println!("Head value: {:?}", list.peek());
    println!("Delete head: {:?}", list.pop());
    println!("Head value: {:?}", list.peek());
}
fn test_graph(){
    let mut graph = graph::Node::new(1, Vec::new());
    println!("Primer nodo id: {}", graph.id);
    let mut graph_2 = graph::Node::new(2, Vec::new());
    let mut graph_3 = graph::Node::new(3, Vec::new());
    graph_2.insert(graph_3.id);
    graph_3.insert(graph.id);
    graph.insert(graph_2.id);
    graph.insert(graph_3.id);
    println!("Primer nodo lista de adyacencia: {:?}", graph.adjacency_list);
    println!("Segundo nodo lista de adyacencia: {:?}", graph_2.adjacency_list);
    println!("Tercer nodo lista de adyacencia: {:?}", graph_3.adjacency_list);
}

fn test_array(){
    let mut arr = array::Array::new(vec![1,2,3]);
    println!("Tamaño del array: {}", arr.size());
    arr.delete(0);
    println!("Tamaño del array: {}", arr.size());
    arr.insert(35);
    println!("Tamaño del array: {}", arr.size());
    println!("El ultimo dato es: {:?}", arr.get(arr.size()-1));
    
    if let Some(value) = arr.get(arr.size() - 1) {
        let new_value = *value - 2;
        println!("El ultimo dato es: {:?}", new_value);
    } else {
        println!("Not found value");
    }

    
}

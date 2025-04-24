
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Estructura para almacenar una fila en la tabla hash, que consiste en una clave y un valor.
pub struct HashRow<K, V> 
where
    K: Hash + Eq + PartialEq // La clave debe ser comparable (Eq) y debe ser hasheable (Hash).
{
    pub key: K,
    pub value: V
}

impl<K, V> HashRow<K, V> 
where
    K: Hash + Eq + PartialEq // La clave debe ser comparable (Eq) y debe ser hasheable (Hash).
{
    // Método para crear un nuevo HashRow con una clave y un valor.
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

// Estructura principal del HashMap que contiene un vector de cubetas (buckets).
pub struct HashMap<K, V>
where
    K: Hash + Eq // La clave debe ser comparable (Eq) y debe ser hasheable (Hash).
{
    buckets: Vec<Vec<HashRow<K, V>>>, // Vec de cubetas, cada cubeta es un Vec de HashRows.
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq // La clave debe ser comparable (Eq) y debe ser hasheable (Hash).
{

    // Método que crea un nuevo HashMap con un número inicial de cubetas.
    pub fn new() -> Self {
        // Se inicializa con 10 cubetas.
        let buckets = (0..10).map(|_| Vec::new()).collect::<Vec<Vec<HashRow<K, V>>>>();
        Self { buckets }
    }

    // Método para insertar una clave y un valor en el HashMap.
    pub fn insert(&mut self, key: K, value: V){
        // Calculamos el hash de la clave.
        let key_hash = hash_calculation(&key);
        // Calculamos el índice del bucket al que pertenece la clave.
        let hash_index = hash_index(key_hash, self.buckets.len());
        
        // Buscamos el bucket correspondiente y añadimos el HashRow.
        if let Some(bucket) = self.buckets.get_mut(hash_index) {
            // Si el bucket ya existe, agregamos el nuevo HashRow.
            bucket.push(HashRow::new(key, value));
        } else {
            panic!("Bucket not found"); // Si no se encuentra el bucket, entra en pánico (esto no debería ocurrir).
        }
    }

    // Método para obtener el valor asociado a una clave.
    pub fn get(&self, key: &K) -> Option<&HashRow<K, V>>{
        // Calculamos el hash de la clave.
        let key_hash = hash_calculation(&key);
        // Calculamos el índice del bucket al que pertenece la clave.
        let hash_index = hash_index(key_hash, self.buckets.len());
        
        // Verificamos si el bucket existe.
        if let Some(bucket) = self.buckets.get(hash_index) {
            // Buscamos en el bucket el HashRow que tenga la misma clave.
            bucket.iter().find(|hash| hash.key == *key)
        } else {
            // Si no se encuentra el bucket, devolvemos None.
            None
        }
    }

    // Método para eliminar una clave del HashMap.
    pub fn remove(&mut self, key: &K){
        // Calculamos el hash de la clave.
        let key_hash = hash_calculation(&key);
        // Calculamos el índice del bucket al que pertenece la clave.
        let hash_index = hash_index(key_hash, self.buckets.len());
        
        // Verificamos si el bucket existe.
        if let Some(bucket) = self.buckets.get_mut(hash_index) {
            // Buscamos la posición del HashRow con la clave que queremos eliminar.
            if let Some(pos) = bucket.iter().position(|row| row.key == *key) {
                // Si lo encontramos, lo eliminamos del bucket.
                bucket.remove(pos);
            }
        }
    }
}

// Función que calcula el valor hash de una clave.
fn hash_calculation<K>(key: K) -> u64
where
    K: Hash,
{
    let mut hasher = DefaultHasher::new(); // Crea un nuevo hasher por defecto.
    key.hash(&mut hasher); // Calcula el hash de la clave.
    hasher.finish() // Devuelve el valor del hash.
}

// Función que calcula el índice del bucket basado en el valor hash y la cantidad de cubetas.
fn hash_index(index: u64, length: usize) -> usize {
    (index as usize) % length // Se calcula el índice tomando el módulo de la longitud de las cubetas.
}


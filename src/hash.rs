use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashRow<K, V> 
where
    K: Hash + Eq + PartialEq
{
    pub key: K,
    pub value: V

}

impl<K, V> HashRow<K, V> 
where
    K: Hash + Eq + PartialEq
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct HashMap<K, V>
where
    K: Hash + Eq
{
    buckets: Vec<Vec<HashRow<K, V>>>,
}


impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq
{

    pub fn new() -> Self {
        // Se inicializa con 10 buckets
        let buckets = (0..10).map(|_| Vec::new()).collect::<Vec<Vec<HashRow<K, V>>>>();
        Self { buckets }
    }

    pub fn insert(&mut self, key: K, value: V){
        let key_hash = hashCalculation(&key);
        let hash_index = hashIndex(key_hash, self.buckets.len());
        if let Some(bucket) = self.buckets.get_mut(hash_index) {
            // Si existe, solo insertamos el nuevo HashRow en el bucket
            bucket.push(HashRow::new(key, value));
        } else {
            panic!("Bucket not found");
        }
    }

    pub fn get(&self, key: &K) -> Option<&HashRow<K, V>>{
        let key_hash = hashCalculation(&key);
        let hash_index = hashIndex(key_hash, self.buckets.len());
        // Verificamos si existe un bucket en ese índice
        if let Some(bucket) = self.buckets.get(hash_index) {
            // Buscamos el HashRow dentro del bucket donde la clave coincida
            bucket.iter().find(|hash| hash.key == *key)
        } else {
            // Si no existe un bucket en esa posición, devolvemos None
            None
        }
    }

    pub fn remove(&mut self, key: &K){
        let key_hash = hashCalculation(&key);
        let hash_index = hashIndex(key_hash, self.buckets.len());
        
        if let Some(bucket) = self.buckets.get_mut(hash_index) {
            // Buscamos el HashRow dentro del bucket donde la clave coincida
            if let Some(pos) = bucket.iter().position(|row| row.key == *key) {
                bucket.remove(pos);
            }
        }
    }
}

fn hashCalculation<K>(key: K) -> u64
where
    K: Hash,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_value = hasher.finish();
    hash_value
}

fn hashIndex(index: u64, length: usize) -> usize {
    (index as usize) % length
}

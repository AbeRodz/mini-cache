use  std::collections::HashMap;
use std::sync::{Arc,Mutex};

#[derive(Debug,Clone)]
pub struct CacheDB{
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl CacheDB {
    pub fn new() -> Self{
        CacheDB {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();

        store.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String>{

        let store = self.store.lock().unwrap();

        store.get(&key).cloned()
    }

    pub fn del(&self, key: String){
        let mut store = self.store.lock().unwrap();

        store.remove(&key);
    }

    pub fn list(&self) -> Vec<String>{
        let store = self.store.lock().unwrap();
        store.keys().cloned().collect()
    }
}
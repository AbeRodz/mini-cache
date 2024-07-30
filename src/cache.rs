
use std::thread;
use  std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::time::{Duration,Instant};

#[derive(Debug,Clone)]
pub struct CacheDB{
    store: Arc<Mutex<HashMap<String, (String,Option<Instant>)>>>,
}

impl CacheDB {
    pub fn new(cleaning_interval : Duration,) -> Self{
        let store = Arc::new(Mutex::new(HashMap::new()));
        let store_clone = store.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(cleaning_interval);
                let now = Instant::now();
                let mut store = store_clone.lock().unwrap();
                store.retain(|_, &mut (_, expiration)| match expiration {
                    Some(expiration_time) => expiration_time > now,
                    None => true,
                });
            }
        });

        CacheDB {
            store,
        }
    }

    pub fn set(&self, key: String, value: String, duration:Option<Duration>) {
        let mut store = self.store.lock().unwrap();
        let expiration = duration.map(|d| Instant::now() +d );
        
        store.insert(key, (value,expiration));
    }
    pub fn get(&self, key: String) -> Option<String>{

        let store = self.store.lock().unwrap();

        store.get(&key).map(|(value,_)| value.clone())
    }

    pub fn get_ttl(&self, key: String) -> Option<Duration>{

        let store = self.store.lock().unwrap();

        store.get(&key).and_then(|(_, expiration)| {
            expiration.map(|exp_time| {
                let now = Instant::now();
                if exp_time > now {
                    exp_time.duration_since(now)
                } else {
                    Duration::new(0, 0)
                }
            })
        })
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
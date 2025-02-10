use {
    std::{
        collections::HashMap,
        sync::{
            Arc,
            Mutex,
        },
        time::Duration,
    },
    tokio::time::sleep,
};

pub struct ExpiringHashMap {
    map: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl ExpiringHashMap {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new())),
        }
    }

    pub fn push_value(&self, key: &str, value: String, ttl: Duration) {
        let mut guard = self.map.lock().unwrap();
        let entry = guard.entry(key.to_string()).or_insert(Vec::new());
        entry.push(value.clone());

        if entry.len() == 1 {
            let map_clone = Arc::clone(&self.map);
            let key = key.to_string();
            tokio::spawn(async move {
                sleep(ttl).await;
                let mut guard = map_clone.lock().unwrap();
                guard.remove(&key);
            });
        }
    }

    pub fn get_values(&self, key: &str) -> Option<Vec<String>> {
        let guard = self.map.lock().unwrap();
        guard.get(key).cloned()
    }
}

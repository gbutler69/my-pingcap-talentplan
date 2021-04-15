use std::collections::HashMap;

pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.storage.get(&key).map(String::clone)
    }
    pub fn remove(&mut self, key: String) {
        self.storage.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

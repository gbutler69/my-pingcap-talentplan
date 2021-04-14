pub struct KvStore;

impl KvStore {
    pub fn new() -> Self {
        todo!()
    }
    pub fn set(&mut self, key: String, value: String) {
        todo!()
    }
    pub fn get(&self, key: String) -> Option<String> {
        todo!()
    }
    pub fn remove(&mut self, key: String) {
        todo!()
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

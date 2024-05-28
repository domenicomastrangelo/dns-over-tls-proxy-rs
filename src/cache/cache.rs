pub trait Cache {
    fn get(&mut self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
    fn del(&mut self, key: &str);
}

pub struct MemoryCache {
    pub cache: std::collections::HashMap<String, String>,
}

impl Cache for MemoryCache {
    fn get(&mut self, key: &str) -> Option<String> {
        self.cache.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.cache.insert(key.to_string(), value.to_string());
    }

    fn del(&mut self, key: &str) {
        self.cache.remove(key);
    }
}

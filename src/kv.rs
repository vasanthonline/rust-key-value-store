use std::collections::HashMap;
use std::option::Option;

pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        return self.map.get(&key).cloned();
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    pub fn to_string(&self) -> String {
        let mut map_string = String::from("");
        for key in self.map.keys() {
            let key_value = format!("{}: {}; ", &key, self.get(key.to_string()).as_deref().unwrap_or(""));
            map_string.push_str(&key_value);
        }
        return map_string;
    }
}
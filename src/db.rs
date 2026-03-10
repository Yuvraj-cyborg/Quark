use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug,Clone)]
pub struct QuarkDB {
    pub store: HashMap<String,String>,
}

impl QuarkDB {
    pub fn new() -> Self {
        QuarkDB {
            store: HashMap::new(),
        }
    }

    pub fn put(&mut self,key: &str,value: &str) -> Result<(), String>{
        match self.store.entry(key.to_string()){
            Entry::Vacant(e) => {
                e.insert(value.to_string());
                Ok(())
            }
            Entry::Occupied(_) => return Err("Key already exists".to_string()),
        }
    }
    pub fn get(&self,key: &str) -> Option<&String> {
        self.store.get(key)
    }
    pub fn update(&mut self,key: &str,value: &str) -> Result<(), String> {
        match self.store.entry(key.to_string()){
            Entry::Occupied(mut e) => {
                e.insert(value.to_string());
                Ok(())
            }
            Entry::Vacant(_) => return Err("Key doesn't exist".to_string()),
        }
    }
    pub fn delete(&mut self,key: &str) -> bool{
        self.store.remove(key).is_some()
    }
}

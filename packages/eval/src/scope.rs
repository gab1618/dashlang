use ast::Value;
use std::collections::HashMap;

pub trait Scope {
    fn get(&self, symbol: String) -> Value;
    fn set(&mut self, symbol: String, val: Value);
}
#[derive(Clone)]
pub struct HashScope {
    memory: HashMap<String, Value>,
}
impl Scope for HashScope {
    fn get(&self, symbol: String) -> Value {
        match self.memory.get(&symbol) {
            Some(value) => value.clone(),
            None => Value::Null,
        }
    }

    fn set(&mut self, symbol: String, val: Value) {
        self.memory.insert(symbol, val);
    }
}

impl HashScope {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
}

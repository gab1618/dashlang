use ast::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait Scope {
    fn get(&self, symbol: &str) -> Value;
    fn set(&self, symbol: &str, val: Value);
}
#[derive(Default)]
pub struct HashScope {
    memory: Rc<RefCell<HashMap<String, Value>>>,
    parent: Option<Rc<dyn Scope>>,
}
impl Scope for HashScope {
    fn get(&self, symbol: &str) -> Value {
        match self.memory.borrow().get(symbol) {
            Some(value) => value.clone(),
            None => match &self.parent {
                Some(parent) => parent.get(symbol),
                None => Value::Void,
            },
        }
    }

    fn set(&self, symbol: &str, val: Value) {
        self.memory.borrow_mut().insert(symbol.to_owned(), val);
    }
}
impl Clone for HashScope {
    fn clone(&self) -> Self {
        Self {
            memory: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Rc::new(Self {
                memory: self.memory.clone(),
                parent: self.parent.clone(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let scope = HashScope::default();
        scope.set("name", Value::String(String::from("John Doe")));

        assert_eq!(scope.get("name"), Value::String(String::from("John Doe")));
    }
    #[test]
    fn test_child_scope() {
        let global = HashScope::default();
        global.set("name", Value::String(String::from("John Doe")));

        let local = global.clone();

        assert_eq!(local.get("name"), Value::String(String::from("John Doe")));
        local.set("name", Value::String(String::from("John Doe jr.")));
        assert_eq!(
            local.get("name"),
            Value::String(String::from("John Doe jr."))
        );
    }
}

use ast::Literal;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait Scope {
    fn get(&self, symbol: &str) -> Literal;
    fn set(&self, symbol: &str, val: Literal);
}
#[derive(Default)]
pub struct HashScope {
    memory: Rc<RefCell<HashMap<String, Literal>>>,
    parent: Option<Rc<dyn Scope>>,
}
impl Scope for HashScope {
    fn get(&self, symbol: &str) -> Literal {
        match self.memory.borrow().get(symbol) {
            Some(value) => value.clone(),
            None => match &self.parent {
                Some(parent) => parent.get(symbol),
                None => Literal::Void,
            },
        }
    }

    fn set(&self, symbol: &str, val: Literal) {
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
        scope.set("name", Literal::String(String::from("John Doe")));

        assert_eq!(scope.get("name"), Literal::String(String::from("John Doe")));
    }
    #[test]
    fn test_child_scope() {
        let global = HashScope::default();
        global.set("name", Literal::String(String::from("John Doe")));

        let local = global.clone();

        assert_eq!(local.get("name"), Literal::String(String::from("John Doe")));
        local.set("name", Literal::String(String::from("John Doe jr.")));
        assert_eq!(
            local.get("name"),
            Literal::String(String::from("John Doe jr."))
        );
    }
}

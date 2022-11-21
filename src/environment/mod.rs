use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Error, Object};

pub struct Environment {
    values: HashMap<String, (Object, bool)>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }
    
    pub fn define(&mut self, name: String, value: Object, is_const: bool) -> Result<(), Error> {
        if is_const && value.is_nil() {
            return Err(Error::Syntax(
                "cannot declare a constant without a value".to_string(),
            ));
        }
        if let Some((_, constant)) = self.values.get(&name) {
            if *constant {
                return Err(Error::Syntax(
                    "cannot reassign a constant variable".to_string(),
                ));
            }
        }
        self.values.insert(name, (value, is_const));
        Ok(())
    }

    pub fn get(&mut self, name: &String) -> Result<Object, Error> {
        if let Some(value) = self.values.get(&name.to_string()) {
            Ok(value.0.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().get(name)
        } else {
            Err(Error::Name(name.to_string()))
        }
    }

    pub fn assign(&mut self, name: &String, value: Object) -> Result<Object, Error> {
        if let Some((_, is_const)) = self.values.get(&name.to_string()) {
            if *is_const {
                return Err(Error::Syntax(
                    "cannot reassign a constant variable".to_string(),
                ));
            }
            self.values.insert(name.to_string(), (value.clone(), *is_const));
            return Ok(value);
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(Error::Name(name.to_string()))
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{Object, Environment};

    #[test]
    fn test_assign() {
        let mut env = Environment::new();
        env.define("a".to_string(), Object::Nil, false).unwrap();
        assert_eq!(env.get(&"a".to_string()).unwrap(), Object::Nil);

        env.assign(&"a".to_string(), Object::Number(1.0)).unwrap();
        assert_eq!(env.get(&"a".to_string()).unwrap(), Object::Number(1.0));
    }

    #[test]
    fn test_const_with_nil() {
        let mut env = Environment::new();
        let out =env.define("a".to_string(), Object::Nil, true);
        assert!(out.is_err());
    }
}
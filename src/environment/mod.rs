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
                "Cannot declare a constant without a value".to_string(),
            ));
        }
        if let Some((_, constant)) = self.values.get(&name) {
            if *constant {
                return Err(Error::Syntax(
                    "Cannot reassign a constant variable".to_string(),
                ));
            }
        }
        self.values.insert(name, (value, is_const));
        Ok(())
    }

    fn get(&mut self, name: &String) -> Result<Object, Error> {
        if let Some(value) = self.values.get(&name.to_string()) {
            Ok(value.0.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().get(name)
        } else {
            Err(Error::Name(name.to_string()))
        }
    }

    fn assign(&mut self, name: &String, value: Object) -> Result<(), Error> {
        if let Some((_, is_const)) = self.values.get(&name.to_string()) {
            if *is_const {
                return Err(Error::Syntax(
                    "Cannot reassign a constant variable".to_string(),
                ));
            }
            self.values.insert(name.to_string(), (value, *is_const));
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(Error::Name(name.to_string()))
        }
    }
}

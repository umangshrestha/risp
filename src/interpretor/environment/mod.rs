use std::collections::HashMap;

use crate::Object;

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}


impl Environment {
    fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    fn get(&mut self, name: &String) -> Result<Object, ErrorInfo> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.get(name)
        } else {
            Err(ErrorInfo::new(
                name.line,
                name.column,
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }
}
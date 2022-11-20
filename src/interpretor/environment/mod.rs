pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}
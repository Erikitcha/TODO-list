use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
}

impl Todo {
    pub fn new(message: String) -> Todo {
        Todo { message }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        Ok(())
    }
}

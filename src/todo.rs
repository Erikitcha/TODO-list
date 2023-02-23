use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
    pub done: bool,
}

impl Todo {
    pub fn new(message: String, done: bool) -> Todo {
        Todo { message, done }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.message, self.done)?;
        Ok(())
    }
}

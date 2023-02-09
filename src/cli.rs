use crate::{terminal::Terminal, todos::Todos};
use console::style;
pub(crate) struct TodoCli {
    terminal: Terminal,
    todos: Todos,
}

impl TodoCli {
    pub fn new(terminal: Terminal, todos: Todos) -> Self {
        TodoCli { terminal, todos }
    }
    pub fn run(&mut self) {
        if let Err(error) = self.terminal.run(self.todos.clone()) {
            println!("{}", style(error.show_error()).red());
        }
    }
}

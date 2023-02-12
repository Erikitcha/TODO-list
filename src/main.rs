use cli::TodoCli;
use crate::terminal::Terminal;
use todos::Todos;
mod cli;
mod terminal;
mod todo;
mod todos;

fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(Todos::new());

    let mut todo_cli = TodoCli::new(user_interface, todo_storage);
    todo_cli.run();
}

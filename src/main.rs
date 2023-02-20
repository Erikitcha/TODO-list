use crate::terminal::Terminal;
use cli::TodoCli;
use todos::Todos;
use console::style;
mod cli;
mod terminal;
mod todo;
mod todos;

fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(Todos::new());

    let mut todo_cli = TodoCli::new(user_interface, todo_storage);

    if let Err(error) = todo_cli.run() {
        println!("{}", style(error.show_error()).red());
    }

}

use cli::TodoCli;
use crate::terminal::Terminal;
use todos::Todos;
mod cli;
mod terminal;
mod todo;
mod todos;

fn main() {
    let terminal = Terminal::new();

    let todos = Todos::new();
    let mut todo_cli = TodoCli::new(terminal, todos);

    todo_cli.run();
}

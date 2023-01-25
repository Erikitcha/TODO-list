use crate::terminal::Terminal;
use console::style;
mod todo;
mod todos;
mod terminal;

fn main() {
    let mut terminal = Terminal::new();

    if let Err(error) = terminal.run() {
        println!("{}", style(error.show_error()).red());
    }
}

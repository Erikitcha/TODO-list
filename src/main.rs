use crate::terminal::Terminal;
use console::style;
mod terminal;
mod todo;
mod todos;

fn main() {
    let mut terminal = Terminal::new();

    if let Err(error) = terminal.run() {
        println!("{}", style(error.show_error()).red());
    }

    /*match terminal.select_option(){
        Ok(value) => println!("{}", value),
        Err(_) => println!("fudeu"),

    }*/
}

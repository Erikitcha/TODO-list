use crate::{terminal:: UserInterface, todos::TodoStorage};
use console::style;
pub(crate) struct TodoCli {
    user_interface: Box<dyn UserInterface>,
    todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn new(user_interface: Box<dyn UserInterface>,  todo_storage: Box<dyn TodoStorage>) -> Self {
        TodoCli { user_interface, todo_storage }
    }
    pub fn run(&mut self) {
        if let Err(error) = self.user_interface.run(self.todo_storage.as_mut()) {
            println!("{}", style(error.show_error()).red());
        }
    }
}

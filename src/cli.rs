use crate::{
    terminal::{TerminalError, UserInterface, UserOption},
    todos::TodoStorage,
};
pub(crate) struct TodoCli {
    user_interface: Box<dyn UserInterface>,
    todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn new(user_interface: Box<dyn UserInterface>, todo_storage: Box<dyn TodoStorage>) -> Self {
        TodoCli {
            user_interface,
            todo_storage,
        }
    }

    pub fn run(&mut self) -> Result<(), TerminalError> {
        self.user_interface.show_hello()?;

        loop {
            self.user_interface.show_menu()?;

            match self.user_interface.select_user_command() {
                Ok(selected_command) => match selected_command {
                    UserOption::NewTodo => {
                        self.user_interface.new_todo(self.todo_storage.as_mut())?
                    }
                    UserOption::RemoveTodo => self
                        .user_interface
                        .remove_todo(self.todo_storage.as_mut())?,
                    UserOption::RemoveAllTodos => self
                        .user_interface
                        .remove_all_todos(self.todo_storage.as_mut())?,
                    UserOption::ShowList => self
                        .user_interface
                        .show_list(self.todo_storage.as_mut().todo_list())?,
                    UserOption::ResolvedTodos => self
                        .user_interface
                        .list_resolved_todos(self.todo_storage.as_mut())?,
                    UserOption::ResolveTodo => self
                        .user_interface
                        .resolve_todo(self.todo_storage.as_mut())?,

                    UserOption::Quit => self.user_interface.quit()?,
                },
                Err(_) => self.user_interface.invalid_input()?,
            }
        }
    }
}

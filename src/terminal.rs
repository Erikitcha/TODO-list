use crate::{todo::Todo, todos::TodoStorage};
use console::{style, Emoji};
use std::io::{Stdin, Stdout, Write};
pub enum TerminalError {
    Stdin(std::io::Error),
    Stdout(std::io::Error),
    InvalidOption,
}

#[derive(Copy, Clone)]
pub enum UserOption {
    NewTodo,
    RemoveTodo,
    ShowList,
    RemoveAllTodos,
    Quit,
}

impl TerminalError {
    pub fn show_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Erro de entrada: {}", style(error).red()),
            TerminalError::Stdout(error) => format!("Erro de saída: {}", style(error).red()),
            TerminalError::InvalidOption => format!(
                "Erro de entrada: {}",
                style("Opção digitada inválida").red()
            ),
        }
    }
}

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

pub trait UserInterface {

    fn new_todo(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError>;

    fn remove_todo(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError>;

    fn remove_all_todos(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError>;

    fn show_list(&mut self, todos: &Vec<Todo>) -> Result<(), TerminalError>;

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError>;

    fn show_todos(&mut self, todos: &Vec<Todo>) -> Result<(), TerminalError>;

    fn show_menu(&mut self) -> Result<(), TerminalError>;

    fn select_user_command(&self) -> Result<UserOption, TerminalError>;

    fn invalid_input(&mut self) -> Result<(), TerminalError>;

    fn show_hello(&mut self) -> Result<(), TerminalError>;

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError>;

    fn read_input(&self) -> Result<String, TerminalError>;

    fn select_from_list(&self, todos: &mut dyn TodoStorage) -> Result<usize, TerminalError>;

    fn quit(&mut self) -> Result<(), TerminalError>;
}

impl UserInterface for Terminal {

    fn new_todo(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError> {
        let new_todo = self.ask_for_new_todo()?;

        if let Some(new_todo) = new_todo {
            todos.add_todo(new_todo.clone());

            self.show_todo(&new_todo)?;
            writeln!(
                self.stdout,
                "{} {}",
                style("\nTODO inserido com sucesso!").cyan(),
                Emoji("🥳", "")
            )
            .map_err(TerminalError::Stdout)?;
        }
        Ok(())
    }

    fn remove_todo(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError> {
        if todos.todo_list().is_empty() {
            self.show_todos(todos.todo_list())?;
            return Ok(());
        }

        println!("Qual TODO gostaria de remover?\n");
        self.show_todos(todos.todo_list())?;

        let selected_index = self.select_from_list(todos)?;
        todos.remove_todo(selected_index);

        println!("TODO removido com sucesso!");
        Ok(())
    }

    fn remove_all_todos(&mut self, todos: &mut dyn TodoStorage) -> Result<(), TerminalError> {
        self.show_todos(todos.todo_list())?;

        todos.remove_all_todos();
        Ok(())
    }

    fn show_list(&mut self, todos: &Vec<Todo>) -> Result<(), TerminalError> {
        self.show_todos(todos)?;
        Ok(())
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        writeln!(
            self.stdout,
            "{}",
            style("Qual TODO gostaria de criar?").blue()
        )
        .map_err(TerminalError::Stdout)?;

        let input = self.read_input()?;
        Ok(Some(Todo::new(input)))
    }

    fn show_todos(&mut self, todos: &Vec<Todo>) -> Result<(), TerminalError> {
        if todos.is_empty() {
            writeln!(self.stdout, "{}", style("A lista está vazia!").red().bold())
                .map_err(TerminalError::Stdout)?;
        }

        for (index, todo) in todos.iter().enumerate() {
            writeln!(
                self.stdout,
                "{} - {}, ",
                style(index).yellow().bold(),
                style(todo.to_owned().message).yellow().bold()
            )
            .map_err(TerminalError::Stdout)?;
        }
        Ok(())
    }

    fn show_menu(&mut self) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "{}",
            style("\n1 - Adicionar novo Todo \n2 - Remover Todo \n3 - Listar Todos \n4 - Esvaziar lista \n5 - Sair\n").green()
        )
        .map(|_| ())
        .map_err(TerminalError::Stdout)
    }

    fn select_user_command(&self) -> Result<UserOption, TerminalError> {
        let input = self.read_input()?;
        let parsed_input = input
            .parse::<usize>()
            .map_err(|_| TerminalError::InvalidOption)?;

        match parsed_input {
            1 => Ok(UserOption::NewTodo),
            2 => Ok(UserOption::RemoveTodo),
            3 => Ok(UserOption::ShowList),
            4 => Ok(UserOption::RemoveAllTodos),
            5 => Ok(UserOption::Quit),
            _ => Err(TerminalError::InvalidOption),
        }
    }

    fn invalid_input(&mut self) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "{}",
            style("\nPor favor, digite uma opção de 1 a 5.\n").red()
        )
        .map_err(TerminalError::Stdout)?;

        Ok(())
    }

    fn show_hello(&mut self) -> Result<(), TerminalError>{
        writeln!(
            self.stdout,
            "{:-^50} {}",
            style("Bem vindo ao TODO List").yellow().bold(),
            Emoji("🌈", "")
        )
        .map_err(TerminalError::Stdout)?;

        Ok(())
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "\nSua mensagem: {}",
            style(&todo.message).green()
        )
        .map(|_| ())
        .map_err(TerminalError::Stdout)
    }

    fn read_input(&self) -> Result<String, TerminalError> {
        let mut buf = String::new();

        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;

        Ok(buf.trim().to_string())
    }

    fn select_from_list(&self, todos: &mut dyn TodoStorage) -> Result<usize, TerminalError> {
        let input = self.read_input()?;
        let size = todos.todo_list().len();
        let parsed_input = input
            .parse::<usize>()
            .map_err(|_| TerminalError::InvalidOption)?;

        if parsed_input > size {
            return Err(TerminalError::InvalidOption);
        }
        Ok(parsed_input)
    }

    fn quit(&mut self) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "{:-^50} ",
            style("OK Finalizando o programa!").blue().bold()
        )
        .map_err(TerminalError::Stdout)?;
        std::process::exit(0);
    }

}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

}

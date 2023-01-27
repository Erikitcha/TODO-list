use crate::{todo::Todo, todos::Todos};
use console::{style, Emoji};
use std::collections::HashMap;
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
            TerminalError::InvalidOption => format!("Erro de entrada: {}", style("Opção digitada inválida").red()),
        }
    }
}
pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
    todos: Todos,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            todos: Todos::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "{} {}",
            style("Bem vindo ao TODO List").yellow().bold(),
            Emoji("🌈", "")
        )
        .map_err(TerminalError::Stdout)?;

        loop {
            self.show_menu()?;

            match self.select_index() {
                Ok(select_index) => self.user_option(select_index)?,
                Err(_) => self.invalid_input()?,
            }
        }
    }

    fn user_option(&mut self, option: UserOption) -> Result<(), TerminalError> {
        match option {
            UserOption::NewTodo => self.new_todo()?,
            UserOption::RemoveTodo => self.remove_todo()?,
            UserOption::RemoveAllTodos => self.remove_all_todos()?,
            UserOption::ShowList => self.show_list()?,
            UserOption::Quit => self.quit()?,
        }
        Ok(())
    }

    fn new_todo(&mut self) -> Result<(), TerminalError> {
        let new_todo = self.ask_for_new_todo()?;

        if let Some(new_todo) = new_todo {
            self.todos.add_todo(new_todo.clone());
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

    fn remove_todo(&mut self) -> Result<(), TerminalError> {
        if self.todos.list.is_empty() {
            writeln!(self.stdout, "Lista de TODOs está vazia.").map_err(TerminalError::Stdout)?;
            return Ok(());
        }

        println!("Qual TODO gostaria de remover?\n");
        self.show_todos("")?;

        let selected_index = self.select_from_list()?;
        self.todos.remove_todo(selected_index);

        println!("TODO removido com sucesso!");
        Ok(())
    }

    fn remove_all_todos(&mut self) -> Result<(), TerminalError> {
        self.show_todos("Removendo todos os TODOS: ")?;
        self.todos.remove_all_todos();
        Ok(())
    }

    fn show_list(&mut self) -> Result<(), TerminalError> {
        self.show_todos("Exibindo todos os TODOS adicionados: ")?;
        Ok(())
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

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "\nSua mensagem: {}",
            style(&todo.message).green()
        )
        .map(|_| ())
        .map_err(TerminalError::Stdout)
    }

    fn show_todos(&mut self, prefix: &str) -> Result<(), TerminalError> {
        let current_elements = self.todos.list.len();
        let mut message = String::from(prefix);

        if current_elements == 0 {
            let empty_prefix = "Lista de TODOs está vazia.".to_owned();
            message.clear();
            message.push_str(&empty_prefix);
        }

        for (index, todo) in self.todos.list.iter().enumerate() {
            message.push_str(&format!("{} - {}, ", index, todo.message));
        }

        writeln!(self.stdout, "{}", style(message).yellow().bold(),)
            .map_err(TerminalError::Stdout)?;
        Ok(())
    }

    fn read_input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();

        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;

        Ok(buf.trim().to_string())
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

    fn option_map(&mut self) -> HashMap<usize, UserOption> {
        let mut option_map = HashMap::new();

        option_map.insert(1, UserOption::NewTodo);
        option_map.insert(2, UserOption::RemoveTodo);
        option_map.insert(3, UserOption::ShowList);
        option_map.insert(4, UserOption::RemoveAllTodos);
        option_map.insert(5, UserOption::Quit);

        return option_map;
    }

    fn select_index(&mut self) -> Result<UserOption, TerminalError> {
        let input = self.read_input()?;

        match input.parse::<usize>() {
            Ok(parsed_input) => {
                if parsed_input > self.todos.list.capacity() + 1 || parsed_input < 1 {
                    return Err(TerminalError::InvalidOption);
                }
                match self.option_map().get(&parsed_input) {
                    Some(user_option) => Ok(user_option.clone()),
                    None => Err(TerminalError::InvalidOption),
                }
            }
            Err(_) => Err(TerminalError::InvalidOption),
        }
    }

    fn select_from_list(&mut self) -> Result<usize, TerminalError> {
        let input = self.read_input()?;

        match input.parse::<usize>() {
            Ok(parsed_input) => {
                for (i, _) in self.todos.list.iter().enumerate() {
                    if i == parsed_input {
                        return Ok(i);
                    }
                }
                return Err(TerminalError::InvalidOption);
            }
            Err(_) => return Err(TerminalError::InvalidOption),
        }
    }

    fn quit(&mut self) -> Result<(), TerminalError> {
        println!("OK Finalizando o programa!");
        std::process::exit(0);
    }

}

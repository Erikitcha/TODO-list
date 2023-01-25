use crate::{todo::Todo, todos::Todos};
use console::style;
use console::Emoji;
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
    ShowTodo,
    RemoveAllTodos,
    Quit
}

impl TerminalError {
    pub fn show_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Erro de entrada: {}", style(error).red()),
            TerminalError::Stdout(error) => format!("Erro de saída: {}", style(error).red()),
            TerminalError::InvalidOption => format!("Erro de entrada: {}", style("Opção digitada invalida").red()),
        }
    }
}
pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
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
            let new_todo = self.ask_for_new_todo()?;
            if let Some(new_todo) = new_todo {
                self.show_todo(&new_todo)?;
                writeln!(
                    self.stdout,
                    "{} {}",
                    style("TODO inserido com sucesso!").cyan(),
                    Emoji("🥳", "")
                )
                .map_err(TerminalError::Stdout)?;
            } else {
                return Ok(());
            }
        }
    }

    fn should_create_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            writeln!(
                self.stdout,
                "{}? ({}/{})",
                style("Deseja criar um novo TODO").magenta(),
                style("s").green(),
                style("n").red()
            )
            .map_err(TerminalError::Stdout)?;

            let input = self.read_input()?;

            match input.as_str() {
                "s" => return Ok(true),
                "n" => return Ok(false),
                _ => {
                    writeln!(
                        self.stdout,
                        "{} {}",
                        style("Entrada inválida").red(),
                        Emoji("🤯", "")
                    )
                    .map_err(TerminalError::Stdout)?;
                }
            }
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        if !self.should_create_todo()? {
            return Ok(None);
        }
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
            "Sua mensagem: {}",
            style(&todo.message).green()
        )
        .map(|_| ())
        .map_err(TerminalError::Stdout)
    }

    fn read_input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();

        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;

        Ok(buf.trim().to_string())
    }
   
    fn show_menu(&mut self) -> Result<(), TerminalError> {
        writeln!(self.stdout, "1 - Adicionar novo Todo \n 2 - Remover Todo \n 3 - Listar Todo \n4 - Listar Todos \n5 - Esvaziar lista \n 6 - Sair").map(|_| ())
        .map_err(TerminalError::Stdout);
        Ok(())
    }

    fn option_map(&mut self) -> HashMap<usize, UserOption>{
        let mut option_map = HashMap::new();

        option_map.insert(1, UserOption::NewTodo);
        option_map.insert(2, UserOption::RemoveTodo);
        option_map.insert(3, UserOption::ShowList);
        option_map.insert(4, UserOption::ShowTodo);
        option_map.insert(5, UserOption::RemoveAllTodos);
        option_map.insert(6, UserOption::Quit);
        option_map
    }

    fn select_option(&mut self) -> Result<UserOption, TerminalError> {
        let input = self.read_input()?;
        match input.parse::<usize>() {
            Ok(parsed_input) => match self.option_map().get(&parsed_input) {
                Some(user_option) => Ok(user_option.clone()),
                None => Err(TerminalError::InvalidOption)
            },
            Err(_) => Err(TerminalError::InvalidOption)
        }   
    }
}

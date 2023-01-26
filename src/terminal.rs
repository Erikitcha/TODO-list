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
                style("Opção digitada invalida").red()
            ),
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

            todos: Todos::new(5),
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
            let user_option = self.select_index()?;
            match user_option {
                UserOption::NewTodo => {
                    let new_todo = self.ask_for_new_todo()?;

                    if let Some(new_todo) = new_todo {
                        self.todos.add_todo(new_todo.clone());

                        self.show_todo(&new_todo)?;
                        writeln!(
                            self.stdout,
                            "{} {}",
                            style("TODO inserido com sucesso!\n").cyan(),
                            Emoji("🥳", "")
                        )
                        .map_err(TerminalError::Stdout)?;
                    }
                }
                UserOption::RemoveTodo => {
                    println!("Qual TODO gostaria de remover?");
                    self.show_todos(String::new())?;

                    let input = self.read_input()?;
                    self.todos.remove_todo(input.parse::<usize>().unwrap());
                    println!("Todo removido com sucesso!\n")
                }
                UserOption::RemoveAllTodos => {
                    self.show_todos("Removendo todos os todos: ".to_owned())?;
                    self.todos.remove_all_todos();
                }
                UserOption::ShowList => {
                    self.show_todos("Exibindo todos os todos: ".to_owned())?;
                }
                UserOption::Quit => {
                    return Ok(());
                }
            }
        }
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
            "Sua mensagem: {}",
            style(&todo.message).green()
        )
        .map(|_| ())
        .map_err(TerminalError::Stdout)
    }

    fn show_todos(&mut self, prefix: String) -> Result<(), TerminalError> {
        let current_elements = self.todos.list.len();
        let mut message = String::from(prefix);

        if current_elements == 0{
            let empty_prefix = "lista vazia".to_owned();
            message.clear();
            message.push_str(&empty_prefix);
        }

        for todo in 0..current_elements{
            message.push_str(&format!("{} - {}, ", todo, self.todos.get_todo(todo).message))
        }
        writeln!(
            self.stdout,
            "{}",
            style(message).yellow().bold(),
        )
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
            style("1 - Adicionar novo Todo \n2 - Remover Todo \n3 - Listar Todos \n4 - Esvaziar lista \n5 - Sair").green()
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
}

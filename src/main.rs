use std::io::{Stdin, Stdout, Write};


fn main() {
    let mut terminal = Terminal::new();

    if let Err(error) = terminal.run() {
        println!("{}", error.show_error());
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    pub fn new(message: String) -> Todo {
        Todo { message }
    }
}

enum TerminalError {
    Stdin(std::io::Error),
    Stdout(std::io::Error),
}

impl TerminalError {
    fn show_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Erro de entrada: {}", error),
            TerminalError::Stdout(error) => format!("Erro de saída: {}", error),
        }
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn run(&mut self) -> Result<(), TerminalError> {
        writeln!(self.stdout, "Bem vindo ao TODO List")
            .map_err(|error| TerminalError::Stdout(error))?;

        loop {
            let new_todo = self.ask_for_new_todo()?;
            if let Some(new_todo) = new_todo {
                self.show_todo(&new_todo)?;
                println!("TODO inserido com sucesso!");
            } else {
                break;
            }
        }
        Ok(())
    }

    fn should_create_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            writeln!(self.stdout, "Deseja criar um TODO? (s/n)")
            .map_err(|error| TerminalError::Stdout(error))?;

            let input = self.read_input()?;

            if input == "s" {
                return Ok(true);
            } else if input == "n" {
                return Ok(false);
            } else {
                println!("Entrada inválida!\nTente novamente com s/n!");
                continue;
            }
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        if !self.should_create_todo()? {
            return Ok(None);
        }
        writeln!(self.stdout, "Qual TODO gostaria de criar?")
            .map_err(|error| TerminalError::Stdout(error))?;
    
        let input = self.read_input()?;
        Ok(Some(Todo::new(input)))
    }
    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "Sua mensagem: {}", todo.message)
            .map(|_| ())
            .map_err(|error| TerminalError::Stdout(error))
    }

    fn read_input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
    
        self.stdin.read_line(&mut buf)
            .map_err(|error| TerminalError::Stdin(error))?;
    
        return Ok(buf.trim().to_string());
    }

}
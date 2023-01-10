use std::io::{ErrorKind, Stdin, Stdout, Write};

fn main() {
    println!("Bem vindo ao TODO List");
    loop {
        println!("Gostaria de criar um novo TODO? (s/n)");
        let mut terminal = Terminal::new();

        match terminal.ask_for_new_todo() {
            Ok(Some(new_todo)) => match terminal.show_todo(&new_todo) {
                Ok(()) => println!("TODO inserido com sucesso!"),
                Err(error) => println!("Erro ao exibir a mensagem TODO: {}", error.show_error()),
            },
            Ok(None) => break,
            Err(error) => {
                println!("{}", error.show_error());
            }
        }
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
    InvalidInput,
}

impl TerminalError {
    fn show_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Erro de entrada: {}", error),
            TerminalError::Stdout(error) => format!("Erro de saída: {}", error),
            TerminalError::InvalidInput => format!("Entrada inválida!\nTente novamente com s/n!"),
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

    fn should_create_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            let mut buf = String::new();

            self.stdin
                .read_line(&mut buf)
                .map_err(|error| TerminalError::Stdin(error))?;
            let input = buf.trim().to_string();

            if input == "s" {
                return Ok(true);
            } else if input == "n" {
                return Ok(false);
            } else {
                return Err(TerminalError::InvalidInput);
            }
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        
        match self.should_create_todo() {
            Ok(should_create) => {
                if !should_create {
                    return Ok(None);
                }
            }
            Err(error) => {
                println!("{}", error.show_error());
            }
        }

        let mut buf = String::new();
        writeln!(self.stdout, "Qual TODO gostaria de criar?")
            .map_err(|error| TerminalError::Stdout(error))?;

        match self.stdin.read_line(&mut buf) {
            Ok(_) => {
                let input = buf.trim().to_string();
                Ok(Some(Todo::new(input)))
            }
            Err(error) => Err(TerminalError::Stdin(error)),
         }
        }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "Sua mensagem: {}", todo.message)
            .map(|_| ())
            .map_err(|error| TerminalError::Stdout(error))
    }
}

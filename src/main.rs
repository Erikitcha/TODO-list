use std::io::{Stdin, Stdout, Write};

fn main() {
    println!("Bem vindo ao TODO List");
    loop {
        println!("Gostaria de criar um novo TODO? (s/n)");
        let mut terminal = Terminal::new();

        if !terminal.should_create_todo() {
            println!("OK Finalizando o programa!");
            break;
        }

        let anwser = terminal.ask_for_new_todo();
        terminal.show_todo(&anwser);
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

    fn should_create_todo(&mut self) -> bool {
        loop {
            let mut buf = String::new();
            self.stdin.read_line(&mut buf).unwrap();
            let input = buf.trim().to_string();

            if input == "s" {
                return true;
            } else if input == "n" {
                return false;
            } else {
                writeln!(self.stdout, "Entrada inválida! Tente novamente com a resposta s ou n!").unwrap();
            }
        }
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        let mut buf = String::new();
        writeln!(self.stdout, "Qual TODO gostaria de criar?").unwrap();
        self.stdin.read_line(&mut buf).unwrap();
        let input = buf.trim().to_string();

        Todo::new(input)
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "Sua mensagem: {}", todo.message).unwrap();
    }
}

use std::io::{Stdin, Stdout, Write};

fn main() {
    println!("Bem vindo ao TODO List");
    loop {
        create_todo()
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

    fn ask_for_new_todo(&mut self) -> Todo {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        let input = buf.trim().to_string();

        Todo::new(input)
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "Sua mensagem: {}", todo.message).unwrap();
    }
}

fn create_todo() {
    let mut terminal = Terminal::new();
    println!("Gostaria de criar um novo TODO? (s/n)");

    let todo = terminal.ask_for_new_todo();
    let anwser: String = todo.message;

    if anwser == "s" {
        println!("Qual TODO gostaria de criar?");

        let new_anwser = terminal.ask_for_new_todo();
        terminal.show_todo(&new_anwser);
    } else if anwser == "n" {
        println!("OK Finalizando o programa!");
        std::process::exit(0)
    } else {
        println!("Entrada inválida! Tente novamente com a resposta s/n!");
    }
}

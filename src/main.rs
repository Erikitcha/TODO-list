fn main() {
    println!("Bem vindo ao TODO List");
    while create_todo() {}
}

#[derive(Debug, Clone)]
struct Todo {
    message: String
}

use std::io::{Stdin, Stdout, Write};

impl Todo{
    pub fn new(message: String) -> Self{
        Self{message}
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {
        // Faça a lógica para construir o Terminal. Nesse caso, você precisará criar uma
        // instância de `Stdin` e `Stdout`. Para isso, utilize as funções:
        //
        // - `std::io::stdin()`
        // - `std::io::stdout()`
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        // Faça a sua lógica para ler a mensagem que será utilizada para construir o
        // `Todo`.
        //
        // Para implementar essa função, você poderá aproveitar parte da lógica da
        // função `input` que apresentamos no projeto anterior. Porém, ao invés de
        // chamar `std::io::stdin()` para invocar o método `.read_line()`, você poderá
        // usar a instância de `Stdin` que já está no próprio terminal. Para isso, você
        // pode fazer `self.stdin.read_line(...)`.
    }

    fn show_todo(&mut self, todo: &Todo) {
        // Faça a sua lógica de impressão aqui. Você poderá usar o macro `writeln!` para
        // isso, por exemplo:
        writeln!(self.stdout, "Sua mensagem: {}", todo.message).unwrap();
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn create_todo() -> bool {
    println!("Gostaria de criar um novo TODO? (s/n)");
    let anwser_todo = input().to_ascii_lowercase();

    if anwser_todo == "s" {
        println!("Qual TODO gostaria de criar?");
        let todo_user = input();
        println!("TODO: {}", todo_user);
        true
    } else if anwser_todo == "n" {
        println!("OK!");
        false
    } else {
        println!("Entrada inválida! Tente novamente com a resposta s/n!");
        true
    }
}

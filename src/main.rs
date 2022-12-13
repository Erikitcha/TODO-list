fn main() {
    println!("Bem vindo ao TODO List");
    create_todo();
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }

fn create_todo(){
    println!("Gostaria de criar um novo TODO? (s/n)");
    let anwser_todo = input();

    if anwser_todo == "s" || anwser_todo == "S"{
        println!("Qual TODO gostaria de criar?");
        let todo_user: String = input();
        println!("TODO: {}", todo_user);
        return main();
    } else if anwser_todo == "n" || anwser_todo == "N"{
        println!("OK!");
    } else {
        return println!("Entrada inválida! Tente novamente com a resposta s/n!");
    }
}

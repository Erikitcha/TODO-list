use crate::todo::Todo;

#[derive(Debug, Clone)]
pub struct Todos {
    pub list: Vec<Todo>,
}

pub trait TodoStorage {
    fn todo_list(&mut self) -> &Vec<Todo>;
    fn get_todo(&mut self, index: usize) -> &Todo;
    fn add_todo(&mut self, todo: Todo);
    fn remove_todo(&mut self, index: usize);
    fn remove_all_todos(&mut self);
}

impl TodoStorage for Todos {
    fn todo_list(&mut self) -> &Vec<Todo> {
        &self.list
    }

    fn add_todo(&mut self, todo: Todo) {
        self.list.push(todo);
    }

    fn get_todo(&mut self, index: usize) -> &Todo {
        self.list.get(index).unwrap()
    }

    fn remove_todo(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn remove_all_todos(&mut self) {
        self.list.clear();
    }
}

impl Todos {
    pub fn new() -> Todos {
        Todos { list: Vec::new() }
    }
}

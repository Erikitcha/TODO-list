use crate::todo::Todo;

#[derive(Debug, Clone)]

pub struct Todos {
    pub list: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Todos {
        Todos {
            list: Vec::with_capacity(15),
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.list.push(todo);
    }

    pub fn remove_todo(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn remove_all_todos(&mut self) {
        for _todo in 0..self.list.len() {
            self.list.pop();
        }
    }
}

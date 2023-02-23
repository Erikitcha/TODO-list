use crate::todo::Todo;

#[derive(Debug, Clone)]
pub struct Todos {
    pub list: Vec<Todo>,
}

pub trait TodoStorage {
    fn todo_list(&mut self) -> &mut Vec<Todo>;
    fn get_todo(&mut self, index: usize) -> &mut Todo;
    fn add_todo(&mut self, todo: Todo);
    fn remove_todo(&mut self, index: usize);
    fn remove_all_todos(&mut self);
    fn resolve_todo(&mut self, index: usize);
}

impl TodoStorage for Todos {
    fn todo_list(&mut self) -> &mut Vec<Todo> {
        &mut self.list
    }

    fn add_todo(&mut self, todo: Todo) {
        self.list.push(todo);
    }

    fn get_todo(&mut self, index: usize) -> &mut Todo {
        self.list.get_mut(index).unwrap()
    }

    fn remove_todo(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn remove_all_todos(&mut self) {
        self.list.clear();
    }

    fn resolve_todo(&mut self, index: usize) {
        let todo = self.get_todo(index);
        todo.done = true;
    }
}

impl Todos {
    pub fn new() -> Todos {
        Todos { list: Vec::new() }
    }
}

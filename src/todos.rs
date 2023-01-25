use crate::todo::Todo;

#[derive(Debug, Clone)]

pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    pub fn new(max_todos: i8) -> Todos {
        Todos {
            todos: Vec::with_capacity(5),
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn get_todo(&mut self, index: usize) -> &Todo {
        return self.todos.get(index).unwrap();
    }

    /*pub fn remove_all_todos(&mut self){
        self.todos.   
    }*/
}

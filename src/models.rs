pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

pub struct NewTodo<'a> {
    pub title: &'a str,
    pub completed: bool,
}

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

pub struct NewTodo<'a> {
    pub title: &'a str,
    pub completed: bool,
}

use juniper::{RootNode, FieldResult};
use rusted_cypher::error::GraphError;
use rusted_cypher::cypher::CypherResult;
use uuid::Uuid;

use crate::models::{Todo};
use crate::db::PrimaryDb;

pub struct Context {
    pub connection: PrimaryDb
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field todoItems(&executor) -> FieldResult<Vec<Todo>> {
        let statement = "MATCH (t:Todo) RETURN t.id, t.title, t.completed";
        let result: CypherResult = executor.context().connection.exec(statement).or_else(|e: GraphError| Err(e))?;
        let mut todos: Vec<Todo> = Vec::new();
        for row in result.rows() {
            let todo = Todo {
                id: row.get("t.id").unwrap(),
                title: row.get("t.title").unwrap(),
                completed: row.get("t.completed").unwrap(),
            };
            todos.push(todo);
        }
        return Ok(todos);
    }
});


pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
        as "Create a new todo item and return it"
    {
        let id: String = Uuid::new_v4().to_string();
        let statement = format!("CREATE (t:Todo {{id: {:?}, title: {:?}, completed: {:?} }}) RETURN t.id, t.title, t.completed", id, &title, completed);
        let result: CypherResult = executor.context().connection.exec(statement).or_else(|e: GraphError| Err(e))?;
        let mut todos: Vec<Todo> = Vec::new();
        for row in result.rows() {
            let todo = Todo {
                id: row.get("t.id").unwrap(),
                title: row.get("t.title").unwrap(),
                completed: row.get("t.completed").unwrap(),
            };
            todos.push(todo);
        }
        let todo: Todo = todos.pop().unwrap();
        return Ok(todo);
    }

    // field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>) -> FieldResult<Option<Todo>>
    //     as "Update an existing todo item.\
    //     \
    //     Will only updated the provided fields - if either `completed` or `title`\
    //     are omitted or null, they will be ignored.\
    //     \
    //     The mutation will return null if no todo item with the specified ID could be found."
    // {
    //     use crate::schema::todos::dsl;
    //     use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
    //
    //     let updated = diesel::update(dsl::todos.find(id))
    //         .set((
    //             completed.map(|completed| dsl::completed.eq(completed)),
    //             title.map(|title| dsl::title.eq(title)),
    //         ))
    //         .execute(&*executor.context().connection)?;
    //
    //     if updated == 0 {
    //         Ok(None)
    //     } else {
    //         Ok(Some(dsl::todos.find(id)
    //             .get_result::<Todo>(&*executor.context().connection)?))
    //     }
    // }
});

pub type Schema = RootNode<'static, Query, Mutation>;

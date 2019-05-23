use juniper::{Context as JuniperContext, FieldResult, FieldError};
use rusted_cypher::error::GraphError;
use rusted_cypher::cypher::CypherResult;

use crate::models::{Todo, NewTodo};
use crate::db::PrimaryDb;

pub struct Context {
    pub connection: PrimaryDb
}

impl JuniperContext for Context {}

graphql_object!(Todo: () |&self| {
    description: "A todo item that can be marked as completed"

    field id() -> i32 as "The unique id of the todo item" {
        self.id
    }

    field title() -> &str as "The user-editable title" {
        &self.title
    }

    field completed() -> bool as "Determines whether the user has completed the item or not" {
        self.completed
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field todoItems(&executor) -> FieldResult<Vec<Todo>> {
        let statement = "MATCH (t:Todo) RETURN t.id, t.title, t.completed";
        let result: Result<CypherResult, GraphError> = executor.context().connection.exec(statement).map_err(Into::into);
        let mut todos: Vec<Todo> = Vec::new();
        for row in result.unwrap().rows() {
            let todo = Todo {
                id: row.get("t.id").unwrap(),
                title: row.get("t.title").unwrap(),
                completed: row.get("t.completed").unwrap(),
            };
            todos.push(todo);
        }
        return Ok(todos);
        // use crate::schema::todos::dsl;
        // use diesel::{RunQueryDsl, QueryDsl};
        //
        // dsl::todos.order(dsl::id)
        //     .load::<Todo>(&*executor.context().connection)
        //     .map_err(Into::into)
    }
});


pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
        as "Create a new todo item and return it"
    {
        let id: i32 = 1256;
        let statement = format!("CREATE (t:Todo {{id: {:?}, title: \"{:?}\", completed: {:?} }}) RETURN t.id, t.title, t.completed", id, &title, completed);
        let result: Result<CypherResult, GraphError> = executor.context().connection.exec(statement).map_err(Into::into);
        // {
        //     Ok(result) => result,
        //     Err(error) => return Result::from_error(FieldError::new(format!({}, error), graphql_value!({ "internal_error": "" }))),
        // };
        let mut todos: Vec<Todo> = Vec::new();
        for row in result.unwrap().rows() {
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

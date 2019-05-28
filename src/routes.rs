use rocket::{get, post};
use rocket::response::content;
use rocket::State;

use crate::neo::{PrimaryDb, Context};
use crate::schema::Schema;

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: PrimaryDb,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: PrimaryDb,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

use rocket_contrib::database;
use rusted_cypher::GraphClient;
use rusted_cypher::error::GraphError;
use rusted_cypher::cypher::CypherResult;
use rusted_cypher::cypher::result::Row;
use juniper::{Executor, FieldResult};

#[database("primary_db")]
pub struct PrimaryDb(pub GraphClient);

pub struct Context {
    pub connection: PrimaryDb
}

impl juniper::Context for Context {}

fn new_from_row<T>(mapper: fn(&Row) -> T, row: &Row) -> T {
    return mapper(row);
}

pub fn call<T>(executor: &Executor<Context>, statement: &str, mapper: fn(&Row) -> T) -> FieldResult<T> {
    let result: CypherResult = executor.context().connection.exec(statement).or_else(|e: GraphError| Err(e))?;
    let mut objects: Vec<T> = Vec::new();
    for row in result.rows() {
        let object: T = new_from_row(mapper, &row);
        objects.push(object);
    }
    let object: T = objects.pop().unwrap();
    return Ok(object);
}

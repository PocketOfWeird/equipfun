use rocket_contrib::database;
use rusted_cypher::GraphClient;

#[database("primary_db")]
pub struct PrimaryDb(pub GraphClient);

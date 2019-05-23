#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate juniper;

use rocket::routes;
use rocket_contrib::serve::{StaticFiles};

mod db;
mod models;
mod routes;
mod schema;

fn main() {
    rocket::ignite()
        .attach(db::PrimaryDb::fairing())
        .manage(schema::Schema::new(
            schema::Query,
            schema::Mutation,
        ))
        .mount("/", routes![
            routes::get_graphql_handler,
            routes::post_graphql_handler,
            routes::graphiql
        ])
        .mount("/", StaticFiles::from("ui/public"))
        .launch();
}

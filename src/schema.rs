use juniper::{RootNode, FieldResult};
use uuid::Uuid;

use crate::models::*;
use crate::neo::{Context, call, call_one};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field manufacturer(&executor, id: Uuid) -> FieldResult<Manufacturer> {
        return call_one(&executor, &Manufacturer::query_single(id), Manufacturer::mapper);
    }
    field manufacturers(&executor) -> FieldResult<Vec<Manufacturer>> {
        return call(&executor, &Manufacturer::query_all(), Manufacturer::mapper);
    }
});


pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field manufacturer_create(&executor, name: String) -> FieldResult<Manufacturer> {
        return call_one(&executor, &Manufacturer::mutate_create(name), Manufacturer::mapper);
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

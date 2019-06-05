use juniper::{RootNode, FieldResult};
use uuid::Uuid;

use crate::types::*;
use crate::neo::{Context, call, call_one};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field manufacturer(&executor, id: Uuid) -> FieldResult<Manufacturer> {
        return call_one(&executor, &Manufacturer::query_single(id), Manufacturer::mapper);
    }
    field manufacturers(&executor) -> FieldResult<Vec<Manufacturer>> {
        return call(&executor, &Manufacturer::query_all(), Manufacturer::mapper);
    }
    field model(&executor, id: Uuid) -> FieldResult<Model> {
        return call_one(&executor, &Model::query_single(id), Model::mapper);
    }
    field models(&executor) -> FieldResult<Vec<Model>> {
        return call(&executor, &Model::query_all(), Model::mapper);
    }
});


pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field manufacturer_create(&executor, name: String) -> FieldResult<Manufacturer> {
        return call_one(&executor, &Manufacturer::mutate_create(name), Manufacturer::mapper);
    }
    field manufacturer_update(&executor, id: Uuid, name: String) -> FieldResult<Manufacturer> {
        return call_one(&executor, &Manufacturer::mutate_update(id, name), Manufacturer::mapper);
    }
    field model_create(&executor, name: String, manufacturer_id: Uuid) -> FieldResult<Model> {
        return call_one(&executor, &Model::mutate_create(name, manufacturer_id), Model::mapper);
    }
    field model_update(&executor, id: Uuid, name: String) -> FieldResult<Model> {
        return call_one(&executor, &Model::mutate_update(id, name), Model::mapper);
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

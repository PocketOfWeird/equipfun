#[derive(juniper::GraphQLObject)]
#[graphql(description="A todo item that can be marked as completed")]
pub struct Todo {
    #[graphql(description="The unique id of the todo item")]
    pub id: String,
    #[graphql(description="The user-editable title")]
    pub title: String,
    #[graphql(description="Determines whether the user has completed the item or not")]
    pub completed: bool,
}

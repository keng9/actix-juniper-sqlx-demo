use juniper::GraphQLObject;
use juniper::GraphQLInputObject;
#[derive(GraphQLObject)]
#[graphql(description = "Human data")]
pub struct Human {
    pub id: i32,
    pub name: String,

}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Human Data")]
pub struct NewHuman {
    pub name: String,
}


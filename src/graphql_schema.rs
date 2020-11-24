use futures::channel::mpsc::channel;
use juniper::{FieldResult, futures};
use juniper::GraphQLInputObject;
use juniper::RootNode;
use sqlx::PgPool;
use crate::models::Human;
use crate::sql::{create_human_data, get_human_data};

#[derive(GraphQLInputObject)]
#[graphql(description = "Human")]
pub struct NewHuman {
    name: String,
}

pub struct Context {
    pub dbpool: PgPool,
}

impl juniper::Context for Context {}
pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    pub async fn GetHuman(context: &Context, name: String) -> FieldResult<Human> {
        Ok(get_human_data(&context.dbpool,name).await?)
    }
}

pub struct MutationRoot;
#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_human_data(context:&Context, name:String)->FieldResult<Human>{
        Ok(create_human_data(&context.dbpool, name).await?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, Subscription {})
}

pub struct Subscription;
#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    pub async fn calls(ctx: &Context) -> CallsStream {
        let (tx, rx) = channel(16);
        Box::pin(rx)
    }
}
type CallsStream = std::pin::Pin<Box<dyn futures::Stream<Item = FieldResult<i32>> + Send>>;

mod graphql_schema;
mod models;
mod sql;


use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware, Error};
use anyhow::Result;
use sqlx::{ PgPool};
use actix_cors::Cors;
use juniper::http::graphiql::graphiql_source;
use std::sync::Arc;
use crate::graphql_schema::{Schema, create_schema};
use juniper::http::GraphQLRequest;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let ctx = graphql_schema::Context {
        dbpool: db_pool.get_ref().to_owned(),
    };

    let res = data.execute(&st, &ctx).await;
    let user = serde_json::to_string(&res)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let address= "0.0.0.0:8080";
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let  pool = PgPool::connect(&database_url).await?;

    let schema = std::sync::Arc::new(create_schema());
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // pass database pool to application so we can access it inside handlers:
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
        .route("/", web::get().to(index))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))

    })
        .bind(address)?
        .run()
        .await?;




    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
    Hello World
    "#,
    )
}


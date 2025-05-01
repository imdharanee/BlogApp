use actix_web::{web, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use tokio;
use crate::schema::{MutationRoot,QueryRoot};

mod db;
mod schema;
mod models;

async fn graphql_handler(
    schema: web::Data<Schema<QueryRoot,MutationRoot, async_graphql::EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connect().await;
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


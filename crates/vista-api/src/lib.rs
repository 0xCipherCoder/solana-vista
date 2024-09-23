pub mod graphql;
pub mod rest;

use actix_web::{web, App, HttpServer, HttpResponse};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use crate::graphql::schema::SolanaVistaSchema;

pub async fn run_server(schema: SolanaVistaSchema) -> std::io::Result<()> {
    println!("GraphQL playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").to(graphql_playground))
            .service(web::resource("/graphql").to(graphql_handler))
            .service(web::resource("/graphql_ws").to(GraphQLSubscription::new(schema.clone())))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql_ws")))
}

async fn graphql_handler(schema: web::Data<SolanaVistaSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
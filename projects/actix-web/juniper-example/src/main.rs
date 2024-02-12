//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, route, web, App, HttpResponse, HttpServer, Responder};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;

use crate::schema::{create_schema, Schema};

#[route("/graphiql", method = "GET")]
async fn graphiql() -> impl Responder {
    let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
    actix_web_lab::respond::Html(html)
}

#[route("/graphql", method = "POST")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = 8080;

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        let state: web::Data<Arc<Schema>> = web::Data::new(schema.clone());
        let cors = Cors::default()
            .allowed_methods(vec!["POST", "GET"])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(state)
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(graphql)
            .service(graphiql)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

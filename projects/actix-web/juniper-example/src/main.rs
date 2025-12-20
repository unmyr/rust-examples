//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use api::{graphiql, graphql};
use schema::{create_schema, Schema};

mod schema;
mod api;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // This std::env::set_var is unsafe in edition 2024
    // See: https://github.com/rust-lang/rust/pull/124636
    // std::env::set_var("RUST_LOG", "actix_web=info");

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

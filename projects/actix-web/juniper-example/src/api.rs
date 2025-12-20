use std::sync::Arc;

use actix_web::{route, web, HttpResponse, Responder};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::Schema;

#[route("/graphiql", method = "GET")]
async fn graphiql() -> impl Responder {
    let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
    web::Html::new(html)
}

#[route("/graphql", method = "POST")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use actix_cors::Cors;
    use actix_web::{
        body::to_bytes, dev::Service, http::{header, StatusCode}, test, web, App, Error
    };
    use crate::schema::{create_schema, Schema};

    #[actix_web::test]
    async fn test_query_graphql() -> Result<(), Error> {
        let schema = std::sync::Arc::new(create_schema());
        let state: web::Data<Arc<Schema>> = web::Data::new(schema.clone());
        let cors = Cors::default()
            .allowed_methods(vec!["POST", "GET"])
            .supports_credentials()
            .max_age(3600);
        let app = test::init_service(
            App::new()
            .app_data(state)
            .service(crate::graphql)
            .service(crate::graphiql)
            .wrap(cors)
        ).await;

        let payload = r#"{"query": "{human(id: \"1234\") {name appearsIn homePlanet}}"}"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/graphql")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload)
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"{"data":{"human":{"name":"Luke","appearsIn":["NEW_HOPE"],"homePlanet":"Mars"}}}"##);

        Ok(())
    }
}

use actix_web::{get, web, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::to_bytes, dev::Service, http::StatusCode, test, App, Error
    };

    use crate::api::index;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = test::init_service(
            App::new()
            .service(index)
        ).await;

        let req = test::TestRequest::get()
            .uri("/1234/john.doe/index.html")
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello john.doe! id:1234"##);

        Ok(())
    }
}

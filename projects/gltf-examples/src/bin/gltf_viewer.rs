use std::fs;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::{Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    let template = include_str!("../../static/index.html");
    let entries = fs::read_dir("./data")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().into_string().ok()?;
            Some(format!(r#"<li><a href="/view/{}">{}</a></li>"#, name, name))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let body = template.replace("<!-- LIST_PLACEHOLDER -->", &entries);
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/data/{name}")]
async fn data_glb_gltf(params: web::Path<String>) -> impl Responder {
    let file_name = params.into_inner();
    if file_name.ends_with(".gltf") {
        HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body(
                fs::read_to_string(format!("data/{}", file_name))
                    .ok()
                    .unwrap(),
            )
    } else if file_name.ends_with(".glb") {
        HttpResponse::Ok()
            .content_type("model/gltf-binary")
            .body(fs::read(format!("data/{}", file_name)).ok().unwrap())
    } else {
        HttpResponse::BadRequest().body("Unexpected file extension.")
    }
}

#[get("/view/{name}")]
async fn view_glb(params: web::Path<String>) -> impl Responder {
    let template = include_str!("../../static/view_glb.html");
    let file_name = params.into_inner();
    let body = template.replace("<!-- GLB_PLACEHOLDER -->", &file_name);
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
            .allowed_origin("https://127.0.0.1:3000")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(data_glb_gltf)
            .service(view_glb)
    })
    .bind("127.0.0.1:3000")?;

    println!("Listening on http://{}", server.addrs().first().unwrap());
    server.run().await
}

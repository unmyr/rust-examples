use std::fs;
use base64::Engine;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::{Responder, get};
use gltf_json::validation::Checked::Valid;
use gltf_json::validation::USize64;

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
    // Vertex data (XYZ)
    let positions: Vec<[f32; 3]> = vec![[0.0, 0.5, 0.0], [-0.5, -0.5, 0.0], [0.5, -0.5, 0.0]];
    let (min, max) = ([-0.5, -0.5, 0.0], [0.5, 0.5, 0.0]);

    // Building the glTF structure (minimal configuration)
    let mut root = gltf_json::Root::default();

    // Convert to a binary buffer
    let mut buffer_data = Vec::new();
    for i in 0..positions.len() {
        let pos = positions[i];
        for v in pos {
            buffer_data.extend_from_slice(&v.to_le_bytes());
        }
    }

    // Buffer definition
    let buffer_length = positions.len() * std::mem::size_of::<[f32; 3]>();
    let custom_engine: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::NO_PAD,
    );
    let buffer = root.push(gltf_json::Buffer {
        byte_length: USize64::from(buffer_length),
        uri: Some(
            "data:application/octet-stream;base64,".to_string()
                + &custom_engine.encode(&buffer_data),
        ),
        extensions: Default::default(),
        extras: Default::default(),
    });

    let buffer_view = root.push(gltf_json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(gltf_json::buffer::Stride(std::mem::size_of::<f32>() * 3)),
        extensions: Default::default(),
        extras: Default::default(),
        target: Some(Valid(gltf_json::buffer::Target::ArrayBuffer)),
    });

    let positions = root.push(gltf_json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(positions.len()),
        component_type: Valid(gltf_json::accessor::GenericComponentType(
            gltf_json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(gltf_json::accessor::Type::Vec3),
        min: Some(gltf_json::Value::from(Vec::from(min))),
        max: Some(gltf_json::Value::from(Vec::from(max))),
        normalized: false,
        sparse: None,
    });

    let primitive = gltf_json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(gltf_json::mesh::Semantic::Positions), positions);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Valid(gltf_json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = root.push(gltf_json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        primitives: vec![primitive],
        weights: None,
    });

    let node = root.push(gltf_json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(gltf_json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        nodes: vec![node],
    });

    fs::create_dir_all("data").expect("Failed to create data directory");
    let writer = fs::File::create("data/triangle.gltf").expect("I/O error");
    gltf_json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

    println!("INFO: Output glTF file completed. : FILE='triangle.gltf'");
    HttpServer::new(|| {
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
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

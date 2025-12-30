use base64::Engine;
use std::fs;

use gltf_json::validation::Checked::Valid;
use gltf_json::validation::USize64;

fn main() {
    let output_file = "triangle_grayscale.gltf";
    // Vertex data (XYZ)
    let vertices: Vec<[f32; 3]> = vec![[0.0, 0.5, 0.0], [-0.5, -0.5, 0.0], [0.5, -0.5, 0.0]];
    let (min, max): ([f32; 3], [f32; 3]) = ([-0.5, -0.5, 0.0], [0.5, 0.5, 0.0]);

    // Building the glTF structure (minimal configuration)
    let mut root = gltf_json::Root::default();

    // Convert to a binary buffer
    let mut buffer_data = Vec::new();
    for pos in &vertices {
        for v in pos {
            buffer_data.extend_from_slice(&v.to_le_bytes());
        }
    }

    // Buffer definition
    let buffer_length = vertices.len() * std::mem::size_of::<[f32; 3]>();
    let buffer = root.push(gltf_json::Buffer {
        byte_length: USize64::from(buffer_length),
        uri: Some(
            "data:application/octet-stream;base64,".to_string()
                + &base64::engine::general_purpose::STANDARD.encode(&buffer_data),
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
        count: USize64::from(vertices.len()),
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
    let writer = fs::File::create(format!("data/{}", output_file)).expect("I/O error");
    gltf_json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

    println!(
        "INFO: The glTF file was successfully generated. : FILE='{}'",
        output_file
    );
}

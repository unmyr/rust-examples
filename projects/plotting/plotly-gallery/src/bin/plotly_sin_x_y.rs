use plotly::{Layout, Plot, Surface};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Handle a single TCP connection
fn handle_connection(mut stream: TcpStream, contents: String) {
    let mut buffer = [0; 1024];

    // Read request into buffer
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Log the request (optional)
            println!("Request: {}", String::from_utf8_lossy(&buffer));

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
                contents.len(),
                contents
            );

            // Send response
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }
}

fn main() {
    let mut z_data = vec![];
    for i in 0..50 {
        let mut row = vec![];
        for j in 0..50 {
            let x = i as f64 / 10.0;
            let y = j as f64 / 10.0;
            row.push((x * y).sin());
        }
        z_data.push(row);
    }

    let surface = Surface::<f64, f64, f64>::new(z_data).name("sin(x*y)");

    let mut plot = Plot::new();
    plot.add_trace(surface);

    let layout = Layout::new().title("<b>3D Surface Plot</b>");
    plot.set_layout(layout);

    // Bind to localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server running at http://127.0.0.1:3000");
    let contents = plot.to_html();

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each connection in a new thread (optional for concurrency)
                let s = contents.clone();
                thread::spawn(move || {
                    handle_connection(stream, s);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        #[allow(unused_variables)]
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

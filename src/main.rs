use std::net::{TcpStream, TcpListener};

struct Config {
    addr: String,
    max_connections: usize,
}

fn run_server(config: Config) {
    let listener = TcpListener::bind(&config.addr).expect("Failed to listen");

    let mut connections: Vec<TcpStream> = Vec::with_capacity(config.max_connections);

    loop {
        match listener.accept() {
            Ok( (stream, addr) ) => {
                if connections.len() >= config.max_connections {
                    eprintln!("max connections reached");
                    continue; // stream is freed
                }

                eprintln!("Accepted connection from: {:?}", addr);
                connections.push(stream);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() {
    let config = Config {
        addr: "127.0.0.1:5000".to_string(),
        max_connections: 3,
    };

    run_server(config);
}

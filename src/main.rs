use std::net::TcpListener;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};

mod connections;
use connections::handle_client;

struct Config {
    addr: String,
    max_connections: usize,
}

fn run_server(config: Config) {
    let listener = TcpListener::bind(&config.addr).expect("Failed to listen");

    let connections = Vec::with_capacity(config.max_connections);
    let connections = Arc::new(Mutex::new(connections));

    loop {

        match listener.accept() {
            Ok( (stream, addr) ) => {
                let mut locked_cons = connections.lock().expect("Failed to lock clients");
                if locked_cons.len() >= config.max_connections {
                    eprintln!("max connections reached");
                    continue; // stream is freed
                }

                eprintln!("Accepted connection from: {:?}", addr);

                // Create a sender / receiver pair and add the sender
                // to the `connections` vector.
                //
                // We use this vector to send messages to all connections
                let (sender, receiver) = channel();
                locked_cons.push(sender);
                drop(locked_cons);

                handle_client(stream, receiver, Arc::clone(&connections));
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

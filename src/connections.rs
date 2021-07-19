use std::thread;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};


pub fn handle_client(
    reader: TcpStream,
    connections: Arc<Mutex<Vec<Sender<String>>>>
) {
    let mut con_locked = connections.lock().expect("Failed to lock connections");

    let (sender, receiver) = channel();
    con_locked.push(sender);
    drop(con_locked);


    let writer = reader.try_clone().expect("Failed to clone the connection");

    thread::spawn(move || {
        read(reader);
    });

    thread::spawn(move || {
        write(writer);
    });
}

fn read(stream: TcpStream) {
}

fn write(stream: TcpStream) {
}

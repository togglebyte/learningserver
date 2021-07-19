use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};


pub fn handle_client(
    stream: TcpStream,
    receiver: Receiver<String>,
    connections: Arc<Mutex<Vec<Sender<String>>>>
) {

}

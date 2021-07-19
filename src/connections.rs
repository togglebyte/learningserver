use std::thread;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};


type Connections = Arc<Mutex<Vec<Sender<String>>>>;

// -----------------------------------------------------------------------------
//     - State -
// First message sets the name,
// all other messages are sent as that (named) user
// -----------------------------------------------------------------------------
enum State {
    Anon,
    User(String),
}

// -----------------------------------------------------------------------------
//     - Reader -
// -----------------------------------------------------------------------------
struct Reader {
    stream: TcpStream,
    state: State,
    buf: Vec<u8>,
    connections: Connections,
}

impl Reader {
    fn new(stream: TcpStream, connections: Connections) -> Self {
        Self {
            stream,
            connections,
            buf: vec![0u8; 1024],
            state: State::Anon,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Writer -
// -----------------------------------------------------------------------------
struct Writer {
    stream: TcpStream,
    receiver: Receiver<String>,
}

impl Writer {
    fn new(stream: TcpStream, receiver: Receiver<String>) -> Self {
        Self {
            stream,
            receiver,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Handle client -
// -----------------------------------------------------------------------------
pub fn handle_client(
    reader: TcpStream,
    connections: Connections,
) {
    let mut con_locked = connections.lock().expect("Failed to lock connections");

    // Add sender to connection list
    let (sender, receiver) = channel();
    con_locked.push(sender);
    drop(con_locked);


    // Clone the tcp stream so we can have both a reader thread and a writer thread
    let writer = reader.try_clone().expect("Failed to clone the connection");

    let reader = Reader::new(reader, connections);
    let writer = Writer::new(writer, receiver);

    thread::spawn(move || { read(reader); });
    thread::spawn(move || { write(writer); });
}

// -----------------------------------------------------------------------------
//     - Read -
// -----------------------------------------------------------------------------
fn read(mut reader: Reader) {
    loop {
        match reader.stream.read(&mut reader.buf) {
            Ok(0) => break,
            Ok(n) => {
                let payload = &reader.buf[..n];
                match std::str::from_utf8(payload) {
                    Ok(s) => {
                        let s = s.trim();
                        match reader.state {
                            State::Anon => reader.state = State::User(s.into()),
                            State::User(ref username) => {
                                let msg = format!("{} > {}\n", username, s);  
                                let mut con = reader.connections.lock().expect("Failed to acquire lock");
                                for sender in con.as_mut_slice() {
                                    let _ = sender.send(msg.clone());
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Invalid utf8 data: {:?}", e),
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {:?}", e);
                break;
            }
        }
    }

    eprintln!("Connection closed (reader)");
}

// -----------------------------------------------------------------------------
//     - Writer -
// -----------------------------------------------------------------------------
fn write(mut writer: Writer) {
    loop {

        let msg = match writer.receiver.recv() {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to receive message: {:?}", e);
                break;
            }
        };

        let res = writer.stream.write_all(msg.as_bytes());
        if let Err(e) = res {
            eprintln!("Failed to write all bytes: {:?}", e);
            break
        }

    }

    eprintln!("Connection closed (writer)");
}

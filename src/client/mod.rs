// client module
extern crate linefeed;

use std::path::Path;
use std::io::{Write, Read};
use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use std::str;
use std::os::unix::net::UnixStream;

use self::linefeed::{Reader, ReadResult};

fn connect_socket(socket_path: &str) -> Option<UnixStream> {
    let socket = Path::new(socket_path);

    match UnixStream::connect(&socket) {
        Ok(stream) => Some(stream),
        Err(_) => None,
    }
}

fn connection_worker(mut stream: UnixStream, tx: mpsc::Receiver<String>) {
    let mut buf = [0; 1024];
    loop {
        if let Ok(data) = tx.recv() {
            match stream.write_all(data.as_bytes()) {
                Ok(_) => {
                    if let Ok(n) = stream.read(&mut buf) {
                        if n != 0 {
                            println!("{}", str::from_utf8(&buf[0..n]).unwrap());
                        }
                    }
                }
                Err(err) => panic!("Cannot write into stream: {}", err),
            }
        }

    }
}

fn spawn_connection_worker(socket_path: &str, rx: mpsc::Receiver<String>) -> Result<(), String> {
    let stream = try!(connect_socket(socket_path).ok_or("Cannot connect to socket"));

    thread::spawn(move || {
        connection_worker(stream, rx);
    });

    Ok(())
}

pub fn start_client(socket_path: &str) {
    let (tx, rx) = mpsc::channel();

    match spawn_connection_worker(socket_path, rx) {
        Err(err) => panic!("{}", err.to_string()),
        Ok(_) => {},
    }

    let mut reader = Reader::new("client").unwrap();
    reader.set_prompt("client> ");

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if line.trim().is_empty() {
            continue;
        } else {
            reader.add_history(line.clone());
        }

        tx.send(line).ok();

        thread::sleep(Duration::new(0, 250)); // XXX: corner-cutting...
    }
}

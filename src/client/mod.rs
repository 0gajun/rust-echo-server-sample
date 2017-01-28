// client module

use std::path::Path;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct Client {
    pub stream: UnixStream,
}

pub trait Send {
    fn send(&mut self, msg: &str) -> usize;
}

impl Send for Client {
    fn send(&mut self, msg: &str) -> usize {
        let buf = msg.as_bytes();

        return match self.stream.write_all(buf) {
            Err(err) => panic!("Cannot write into stream: {}", err),
            Ok(_) => {
                self.stream.flush();
                buf.len()
            }
        }
    }
}

pub fn connect(socket_path: &str) -> Client {
    let socket = Path::new(socket_path);

    return match UnixStream::connect(&socket) {
        Err(_) => panic!("server is not running"),
        Ok(stream) => Client { stream: stream }
    };
}



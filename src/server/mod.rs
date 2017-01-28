// server module
extern crate mio_uds;
extern crate mio;

use std::path::Path;
use std::io::{Read, Write};
use std::str;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::fs;

use self::mio::*;
use self::mio_uds::*;

const MAX_CLIENT: usize = 10000;
const MAX_CLIENT_TOKEN: Token = Token(MAX_CLIENT);

const SERVER: Token = Token(0);
const CLIENT_START: Token = Token(1);

struct ClientHolder {
    clients: HashMap<Token, UnixStream>,
    free_tokens: Vec<Token>,
    next_max_token: Token,
}

impl ClientHolder {
    fn new_from(start_token: Token) -> ClientHolder {
        ClientHolder {
            clients: HashMap::new(),
            free_tokens: Vec::new(),
            next_max_token: start_token,
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        match self.free_tokens.pop() {
            Some(token) => return Some(token),
            None => {}
        }

        let next = Token::from(usize::from(self.next_max_token) + 1);
        if next >= MAX_CLIENT_TOKEN {
            // Exceeded max client limit
            return None;
        }

        let result = self.next_max_token.clone();
        self.next_max_token = next;
        Some(result)
    }

    fn get_mut_stream(&mut self, token: Token) -> Option<&mut UnixStream> {
        self.clients.get_mut(&token)
    }

    fn register(&mut self, token: Token, stream: UnixStream) {
        self.clients.insert(token, stream);
    }

    fn unregister(&mut self, token: Token) {
        self.clients.remove(&token);
        self.free_tokens.push(token);
    }
}

pub fn start_server(socket_path: &str) {
    fs::remove_file(socket_path);
    let socket = Path::new(socket_path);

    let listener = match UnixListener::bind(socket) {
        Err(_) => panic!("Cannot bind socket"),
        Ok(listener) => listener,
    };

    let mut client_holder = ClientHolder::new_from(CLIENT_START);
    let poll = Poll::new().unwrap();

    poll.register(&listener, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    match listener.accept() {
                        Ok(Some((stream, sock_addr))) => {
                            println!("Accepted!");

                            let token = client_holder.get_next_token().unwrap();
                            poll.register(&stream, token, Ready::readable(), PollOpt::edge())
                                .unwrap();
                            client_holder.register(token, stream);
                        }
                        Ok(None) => {
                            println!("No incoming connection");
                        }
                        Err(err) => panic!("{}", err.to_string()),
                    }
                }
                token => {
                    let mut closed = false;
                    if let Some(stream) = client_holder.get_mut_stream(token) {
                        let mut buf = [0; 1024];

                        let size = match stream.read(&mut buf) {
                            Ok(size) => size,
                            Err(err) => continue,
                        };

                        if size == 0 {
                            closed = true;
                            poll.deregister(stream).unwrap();
                            println!("disconnected");
                        } else {
                            stream.write(&buf[0..size]);
                            println!("handled: {}", str::from_utf8(&buf).unwrap())
                        }
                    }
                    if closed {
                        client_holder.unregister(token);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

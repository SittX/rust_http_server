use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use std::fs::read;
use std::io::BufReader;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Can't listen at the given port");
    let mut client_handler = ClientHandler::new();
    println!("server listening on port : {}", 8080);

    let handler = thread::spawn(move || {
        loop {
            let (mut client_socket, addr) = listener.accept().unwrap();
            client_handler.add(client_socket.try_clone().unwrap());
            println!("** {} is connected.", addr.ip());
            println!(
                "Total number of client connected : {}",
                client_handler.list_of_client.lock().unwrap().len()
            );
            thread::spawn(move || {
                loop {
                    let mut buffer = [0u8; 1024];
                    let bytes_read = client_socket
                        .read(&mut buffer)
                        .expect("Cannot read data from the client.");

                    if bytes_read == 0{
                        println!("Client {} is disconnected.",addr.ip());
                        break;
                    }
                    println!("Number of bytes read from the client : {} ", bytes_read);

                    client_socket
                        .write_all(&buffer[..bytes_read])
                        .expect("Cannot write to the client socket.");
                }
            });
        }
    });
    handler.join().unwrap();
}

struct ClientHandler {
    list_of_client: Mutex<Vec<TcpStream>>,
}

impl ClientHandler {
    fn new() -> Self {
        Self {
            list_of_client: Mutex::new(Vec::new()),
        }
    }

    pub fn add(&mut self, client_socket: TcpStream) {
        self.list_of_client.lock().unwrap().push(client_socket);
    }
}

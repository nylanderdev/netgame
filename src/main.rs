#[cfg(test)]
mod test;
#[macro_use]
extern crate lazy_static;

use crate::game::Client;
use crate::net::{Connection, SmartProtocol};
use std::net::{TcpListener, TcpStream};

mod game;
mod misc;
mod net;

use crate::game::Server;
use std::process::exit;

fn main() {
    // Skip the first command-line argument, it's just the working dir
    let args: Vec<String> = std::env::args().skip(1).collect();
    // User needs to submit an address or the hosting option "-host"
    if args.is_empty() {
        eprintln!("Usage error");
        exit(1);
    }
    if args[0] == "-host" {
        // Spawn the server in another thread and connect to localhost
        std::thread::spawn(server_main);
        client_main(&"localhost".to_string())
    } else {
        // Connect to a remote host
        client_main(&args[0])
    }
}

/// Set up a server at port 1337 and any address
fn server_main() {
    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();
    // Connect to clients
    let (remote1, _address) = listener.accept().unwrap();
    let (remote2, _address) = listener.accept().unwrap();
    // Wrap clients in connections and protocols to enable Event exchange
    let conn1 = Connection::<SmartProtocol>::from_socket(remote1);
    let conn2 = Connection::<SmartProtocol>::from_socket(remote2);
    // Start the server up
    let mut server = Server::new(conn1, conn2);
    server.main();
}

/// Connect to a server at port 1337 and the given address
fn client_main(address: &String) {
    // Connect to a local or remote host at port 1337
    let remote = TcpStream::connect(format!("{}:1337", address)).unwrap();
    // Wrap the server to enable easy de/serialization
    let conn = Connection::<SmartProtocol>::from_socket(remote);
    // Create a client instance -- Though the server may not have connected the other client yet
    let client = Client::new();
    client.main(conn);
}

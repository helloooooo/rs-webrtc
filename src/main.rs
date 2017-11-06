extern crate websocket;

use std::thread;
use std::sync::{Arc, Mutex};
use websocket::OwnedMessage;
use websocket::sync::Server;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    let ip_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        let ip_list = ip_list.clone();
        println!("testydsadasda");
        thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }
            let mut client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap().to_string();
            println!("Connection from {}", &ip);
            let mut ip_list = ip_list.lock().unwrap();
            ip_list.push(ip);
            let length = ip_list.len();
            println!("{}", length);
            println!("testy");
            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();
            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();
                println!("{:?}", &message);
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}

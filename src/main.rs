/// Example showing how to obtain the ip address of the client, where possible.

extern crate ws;
extern crate serde;
extern crate serde_json;


use ws::{Sender, Factory, Handler, WebSocket, listen, Message};
#[macro_use]
extern crate serde_derive;

use serde_json::Error;



struct MyHandler {
    ws: Sender,
    is_server: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SDP_UUID {
    text: String,
    uuid: String,
}

impl Handler for MyHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(ip_addr) = try!(shake.remote_addr()) {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("{}", &msg);
        if let Ok(text) = msg.into_text() {
            match serde_json::from_str::<SDP_UUID>(&text) {
                Ok(status) => self.ws.broadcast(serde_json::to_string(&status).unwrap()),
                Err(e) => self.ws.broadcast(text),
            };
        }
        Ok(())
    }
}

struct MyFactory;

impl Factory for MyFactory {
    type Handler = MyHandler;

    fn connection_made(&mut self, ws: Sender) -> MyHandler {
        MyHandler {
            ws: ws,
            // default to client
            is_server: false,
        }
    }

    fn server_connected(&mut self, ws: Sender) -> MyHandler {
        MyHandler {
            ws: ws,
            is_server: true,
        }
    }
}


fn main() {
    let mut wes = WebSocket::new(MyFactory {}).unwrap();
    wes.listen("127.0.0.1:3012").unwrap();
}

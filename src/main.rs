extern crate ws;

use ws::listen;

fn main() {
    if let Err(error) = listen("127.0.0.1:3012", |out| {
        move |msg| {
            println!("server get message '{}'", msg);
            out.send(msg)
        }
    })
    {
        println!("Failed to create websocket due to {:?}", error);
    }
}

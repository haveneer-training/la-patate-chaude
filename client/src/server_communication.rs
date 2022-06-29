use std::io::Write;
use std::net::TcpStream;
use common::models::Message;

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).unwrap();
    let serialized_size = serialized.len() as u32;
    println!("sending message serialized ({serialized_size:?}) : {serialized:?}");

    stream.write_all(&serialized_size.to_be_bytes()).unwrap();
    stream.write_all(&serialized.as_bytes()).unwrap();
}
use std::io::{Read, Write};
use std::net::{TcpStream};

use common::models::{Message, Subscribe, SubscribeResult, Welcome};

fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).unwrap();
    let serialized_size = serialized.len() as u32;
    println!("sending message serialized ({serialized_size:?}) : {serialized:?}");

    stream.write_all(&serialized_size.to_be_bytes()).unwrap();
    stream.write_all(&serialized.as_bytes()).unwrap();
}

fn on_welcome(stream: &TcpStream, welcome: Welcome) {
    println!("welcome: {welcome:?}");
    let subscribe = Subscribe { name: String::from("Paul") };
    send_message(stream, Message::Subscribe(subscribe));
}

fn on_subscribe_result( subscribe_result: SubscribeResult) {
    println!("subscribe_result: {subscribe_result:?}");
}

fn main_loop(mut stream: &TcpStream){

    let mut buf = [0; 4];

    send_message(stream, Message::Hello);

    println!("Listening");

    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => { continue; }
        }
        println!("receiving message");

        let message_size = u32::from_be_bytes(buf);
        println!("message_size: {message_size:?}");

        let mut message_buf = vec![0; message_size as usize];
        stream.read_exact(&mut message_buf).expect("TODO: panic message");

        let message = std::str::from_utf8(&message_buf).expect("TODO: panic message");
        println!("message: {message:?}");

        let record : Message = serde_json::from_str(&message).expect("TODO: panic message");
        println!("message: {record:?}");

        match record {
            Message::Hello => {}
            Message::Welcome(welcome) => { on_welcome(stream, welcome)}
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => { on_subscribe_result( subscribe_result); }
        }
    }

}

fn main() {
    println!("Hello, world!");

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(stream ) => {
          main_loop(&stream);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

}

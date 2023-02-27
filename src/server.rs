use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::Deserializer;
use serde_json::Value;

mod message;
use message::Message;

fn json_to_message(json: String)->Message{
    serde_json::from_str(&json.as_str()).unwrap()
}

fn handle_client(mut stream: TcpStream) {
    let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
    stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

    let len = u32::from_le_bytes(buf_len); // convertit les 4 octets en un entier u32

    let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
    stream.read_exact(buf.as_mut()).unwrap(); // on remplit le buffer
    // c'est arrivé
    println!("{buf:?}"); // en octets
    let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
    //println!("{s}"); // en String
    let message = json_to_message(s.to_string());
    println!("struct rust {:?}", message);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

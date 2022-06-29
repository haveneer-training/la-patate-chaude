mod server_communication;
mod player_init;

use std::io::Read;
use std::net::{TcpStream};

use common::models::{Challenge, EndOfGame, Message, PublicPlayer, RoundSummary};
use crate::player_init::{on_subscribe_result, on_welcome};
use crate::server_communication::send_message;


fn on_leader_board(leader_board: Vec<PublicPlayer>){
    println!("LeaderBoard: {leader_board:?}");
}

fn on_challenge(stream: &TcpStream, challenge: Challenge){

}

fn on_round_summary(stream: &TcpStream, summary: RoundSummary){
    println!("RoundSummary: {summary:?}");

}

fn on_end_of_game(end_of_game: EndOfGame){
    println!("EndOfGame: {end_of_game:?}");
}

fn main_loop(mut stream: &TcpStream, name: &String){

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
        stream.read_exact(&mut message_buf).expect("failed to read message");

        let record = buffer_to_object(&mut message_buf);

        match record {
            Message::Hello => {}
            Message::Welcome(welcome) => { on_welcome(stream, welcome, name)}
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => { on_subscribe_result( subscribe_result); }
            Message::PublicLeaderBoard(leader_board) => { on_leader_board(leader_board); }
            Message::Challenge(challenge) => { on_challenge(stream, challenge);}
            Message::RoundSummary(summary) => {on_round_summary(stream, summary);}
            Message::EndOfGame(end_of_game) => {on_end_of_game(end_of_game); break;}
        }
    }

}

fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");
    println!("message: {message:?}");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    println!("message: {record:?}");
    record
}

fn main() {
    let name = std::env::args().nth(1).expect("no name given");

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(stream ) => {
          main_loop(&stream, &name);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

}

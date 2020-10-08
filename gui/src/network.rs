use chess_engine::pieces::default::*;
use chess_engine::Board;
use chess_engine::Team;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread::{spawn, JoinHandle};

pub enum Move {
    Standard(i64, i64),
    EnPassant(i64, i64),
    Promotion(i64, i64, i64),
    KingsideCastling,
    QueensideCastling,
    Other,
}

pub enum MessageType {
    Decline,
    Move(Move),
    Undo,
    Accept,
    Checkmate,
    Draw, 
    Resign, 
    Other,
}

pub struct Net{
    host: bool,
    ip: String,
    has_received: bool,
    read_data: String, 
}

impl Net{
    pub fn new(host: bool, ip: String) -> Net{  
        let mut receive = false; 
        if host == true{
            receive = true; 
        }
        Net{
            host: host, 
            ip: ip,
            has_received: receive,
            read_data: "".to_string(), 
        }
    }

    pub fn connect(&mut self){
        if self.host == true{
            self.start_host(); 
        }
        else{
            self.start_client(); 
        }
    }

    pub fn start_client(&mut self) {
        match TcpStream::connect(self.ip.to_owned()) {
            Ok (stream) => {   
                loop{
                    self.handle_client(stream.try_clone().ok().unwrap()); 
                } 
            },
            Err(_) => {
                //
            }
        }
    }

    pub fn start_host(&mut self) {
        let listener = TcpListener::bind(self.ip.to_owned()).unwrap(); 
        println!("Server listening on {}", self.ip); 

        for stream in listener.incoming() {
            loop{
                self.handle_client(stream.try_clone().ok().unwrap()); 
            }
        }

        drop(listener);
    }

    pub fn handle_client(&mut self, mut stream: TcpStream){
        if self.has_received == true{
            println!("Data: {:?}", self.read_data);
            let msg = b"Hello!";
            stream.write(msg).unwrap(); 
            self.has_received = false; 
        }
        else{
            println!("Receiving data");
            self.receive_message(stream); 
            println!("Done receiving data"); 
            self.has_received = true; 
        }
    }

    pub fn receive_message(&mut self, mut stream: TcpStream) {
        let mut data = [0 as u8; 6]; 
        match stream.read(&mut data) {
            Ok(size) => {
                let mut text = from_utf8(&data).unwrap().to_string().to_owned();
                text.pop(); 
                println!("Size: {}, Text: {}", size, text); 
                self.read_data = text;
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

pub fn parse_message(message: [u8, 32]) -> String{
    match message[0] {
        0 => MessageType::Decline, 
        1 => {
            MessageType::Move(match message[1] {
                0 => Move::Standard(message[2], message[3]),
                1 => Move::Standard(message[2], message[3]),
                2 => Move::Promotion(message[2], message[3], message[4]),
                3 => Move::KingsideCastling,
                4 => Move::QueensideCastling,
                _ => Move::Other,
            })
        }
        2 => MessageType::Undo,
        3 => MessageType::Accept,
        4 => MessageType::Checkmate,
        5 => MessageType::Draw,
        6 => MessageType::Resign,
        _ => MessageType::Other,
    }
}

pub fn encode_message(type: MessageType) {
    match type{
        Decline => 0, 
        Move() => 1,
        Undo => 2, 
        Accept => 3,
        Checkmate => 4,
        Draw => 5,
        Resign => 6,
        _ => -1, 
    }
}
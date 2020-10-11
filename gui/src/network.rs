use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::mpsc;

pub enum Move {
    Standard(u8, u8),
    EnPassant(u8, u8),
    Promotion(u8, u8, u8),
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
    stream: TcpStream, 
    rx: mpsc::Receiver<MessageType>, 
}

impl Net{
    pub fn connect(host: bool, addr: String) -> Self{
        if host == true{
            Self::init_host(host, addr) 
        }
        else{
            Self::init_client(host, addr)
        }
    }

    pub fn init_client(host: bool, addr: String) -> Self{
        let (tx, rx) = mpsc::channel::<MessageType>(); 
        let stream = TcpStream::connect(addr.to_owned()).ok().unwrap(); 
        let mut socket = stream.try_clone().ok().unwrap(); 
        let mut buffer = [0; 32]; 
        thread::spawn(move || {
            while let Ok(message) = socket.read(&mut buffer) {
                if message == 0{break;}
                tx.send(decode_message(buffer)).expect("Error while sending message"); 
            }
        }); 

        Self {
            host,
            stream,
            rx, 
        }
    }

    pub fn init_host(host: bool, addr: String) -> Self{
        let (tx, rx) = mpsc::channel::<MessageType>(); 

        let listener = TcpListener::bind(addr.to_owned()).unwrap(); 
        println!("Listening on {}", addr); 

        let (mut socket, _) = listener.accept().unwrap();
        println!("Client has connected to {}", addr); 

        let stream = socket.try_clone().ok().unwrap(); 

        let mut buffer = [0; 32]; 
        thread::spawn(move || {
            while let Ok(message) = socket.read(&mut buffer) {
                if message == 0{break;}
                tx.send(decode_message(buffer)).expect("Error while sending message"); 
            }
        });

        Self {
            host,
            stream,
            rx, 
        }
    }
 
    pub fn receive_message(&mut self) -> MessageType {
        //println! ("Message recieved"); 
        let mut message = MessageType::Other; 
        while let Ok(msg) = self.rx.try_recv(){
            message = msg; 
        }
        message
    }

    pub fn send_message(&mut self, message: MessageType) {
        self.stream.write(&encode_message(message)).expect("Failed writing"); 
    } 

    pub fn is_host(&self) -> bool { 
        self.host
    }
}

pub fn decode_message(message: [u8; 32]) -> MessageType{
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

pub fn encode_message(message: MessageType) -> Vec<u8> {
    let mut encoded_message = Vec::<u8>::new();  
    match message {
        MessageType::Decline =>  encoded_message.push(0), 
        MessageType::Move(mv) =>  {
            match mv {
                Move::Standard(pos1, pos2) => encoded_message.extend_from_slice(&[1, 0, pos1, pos2]),
                Move::EnPassant(pos1, pos2) => encoded_message.extend_from_slice(&[1, 1, pos1, pos2]),
                Move::Promotion(pos1, pos2, piece_type) => encoded_message.extend_from_slice(&[1, 2, pos1, pos2, piece_type]),
                Move::KingsideCastling => encoded_message.extend_from_slice(&[1, 3]),
                Move::QueensideCastling => encoded_message.extend_from_slice(&[1, 4]),
                _ => (), 
            }
        } 
        MessageType::Undo =>  encoded_message.push(2), 
        MessageType::Accept =>  encoded_message.push(3), 
        MessageType::Checkmate =>  encoded_message.push(4), 
        MessageType::Draw =>  encoded_message.push(5), 
        MessageType::Resign =>  encoded_message.push(6), 
        _ => (), 
    }
    encoded_message
}

pub fn decode_position(pos: u8) -> (i64, i64) {
    (pos as i64 % 8, pos as i64 / 8)
}

pub fn encode_position(x: i64, y: i64) -> u8 {
    x as u8 + (y as u8 * 8)
} 
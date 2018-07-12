extern crate tokio;
extern crate futures;
extern crate tokio_codec;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate bytes;

use bincode::{deserialize, serialize};

use tokio::prelude::*;
use tokio::io;
use tokio::net;
use tokio_codec::*;
use bytes::{BufMut, BytesMut};

use std::env;

fn main() {
    let a = env::args().skip(1).collect::<Vec<_>>();
    match a.first().unwrap().as_str() {
        "client" => client(),
        "server" => server(),
        _ => panic!("failed"),
    };
}


struct MessageCodec {
    vec_length: u64,// Length of the receive vector
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    processMsg(Process),
    seedMsg(Seed),
    stateMsg(Sate),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Process {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Seed {
    x: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sate {
    y: Vec<f32>,
}

impl MessageCodec {
    fn new() -> MessageCodec {
        MessageCodec { vec_length: 0 }
    }

    pub fn number_to_four_vecu8(num: u64) -> Vec<u8> {
        assert!(num < (1 << 32));
        let mut result: Vec<u8> = vec![];
        let mut x = num;
        loop {
            if x / 256 > 0 {
                result.push((x % 256) as u8);
                x = x / 256;
            } else {
                result.push((x % 256) as u8);
                break;
            }
        }
        for _ in 0..(4 - result.len()) {
            result.push(0);
        }
        result.reverse();
        return result;
    }


    pub fn four_vecu8_to_number(vec: Vec<u8>) -> u64 {
        assert_eq!(vec.len(), 4);
        let num = vec[0] as u64 * 256 * 256 * 256 + vec[1] as u64 * 256 * 256
            + vec[2] as u64 * 256 + vec[3] as u64;
        return num;
    }
}

impl Encoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut temp = serialize(&item).unwrap();
        let mut encoder: Vec<u8> = MessageCodec::number_to_four_vecu8(temp.len() as u64);
        encoder.append(&mut temp);
        dst.put(encoder);
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            Ok(None)
        } else {
            let mut vec: Vec<u8> = src.to_vec();
            let truth_data = vec.split_off(4);
            let vec_length = MessageCodec::four_vecu8_to_number(vec);
            // assert!(self.vec_length > 0);
            if truth_data.len() == vec_length as usize {
                let msg: Message = deserialize(&truth_data).unwrap();
                src.take();
                Ok(Some(msg))
            } else {
                Ok(None)
            }
        }
    }
}


fn server() {
    let socket_addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let listener = net::TcpListener::bind(&socket_addr).unwrap();
    let done = listener.incoming().for_each(|tcp_stream| {
        let framed = MessageCodec::new().framed(tcp_stream);
        let (writer, reader) = framed.split();
        let process = reader.for_each(|a| {
            println!("{:?}", a);
            Ok(())
        }).map_err(|e| { println!("{:?}", e); });
        tokio::spawn(process);
        Ok(())
    }).map_err(|e| { println!("{:?}", e); });
    tokio::run(done);
}

fn client() {
    let addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let mut tcp_connect = net::TcpStream::connect(&addr);
    let s = tcp_connect.map(|mut stream| {
        let msg: Message = Message::seedMsg(Seed { x: 333 });
        let mut buf = BytesMut::new();
        MessageCodec::new().encode(msg, &mut buf);
        println!("{},{:?}", buf.len(), buf);
        stream.write_all(&buf.to_vec()[..]).unwrap()
    }).map_err(|e| { println!("{:?}", e); });
    tokio::run(s);
}
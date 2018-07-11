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
//    let a = env::args().skip(1).collect::<Vec<_>>();
//    match a.first().unwrap().as_str() {
//        "client" => client(),
//        "server" => server(),
//        _ => panic!("failed"),
//    };
}

struct MessageCodec {
    vec_length: u32,// Length of the receive vector
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

    fn number_to_two_vecu8(num: u32, &mut vec: Vec<u8>) {
        assert!(num >= (1 << 16));
        let vec = vec![(num / 256) as u8, (num % 256) as u8];
    }

    fn two_vecu8_to_number(vec: Vec<u8>, &mut num: u32) {
        assert_eq!(vec.len(), 2);
        let num = (vec[0] * 256 + vec[1]) as u32;
    }
}

impl Encoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let x = serialize(&item).unwrap();
        let encoder =
        let dst = BytesMut::from(x);
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(deserialize(&src.to_vec()).unwrap())
    }
//    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>
//    {}
}


fn server() {
    let socket_addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let listener = net::TcpListener::bind(&socket_addr).unwrap();
//    let done = listener.incoming().for_each(|tcp_stream| {
//        let framed = MessageStream.framed(tcp_stream);
//        let (_writer, reader) = framed.split();
//        reader.for_each();
//    });
}

fn client() {
    let addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let mut tcp_connect = net::TcpStream::connect(&addr);
}
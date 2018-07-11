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

fn main() {
    client();
}

struct MessageStream;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    processMsg(Process),
    seedMsg(Seed),
    stateMsg(Sate),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Process {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Seed {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sate {}

impl Encoder for MessageStream {
    type Item = Message;
    type Error = io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error>
    {
        let dst = BytesMut::from(serialize(&item).unwrap());
        Ok(())
    }
}

impl Decoder for MessageStream {
    type Item = Message;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>
    {
        Ok(deserialize(&src.to_vec()).unwrap())
    }
}


#[test]
fn server() {
    let socket_addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let listener = net::TcpListener::bind(&socket_addr).unwrap();
    // let listener = network::create_uds_listener("socket");
    let server = listener.incoming().for_each(move |stream| {
        let read_future = io::read_to_end(stream, Vec::new())
            .and_then(|(_, bytes)| {
                println!("{:?}", bytes);
                let (tx, rx) = futures::sync::mpsc::unbounded();
                tx.unbounded_send("2").unwrap();
                println!("sended");
                Ok(())
            }).map_err(|e| println!("{:?}", e));
        tokio::spawn(read_future);
        Ok(())
    }).map_err(|e| println!("{:?}", e));
    tokio::run(server);
}

//fn client() {
//    let addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
//    let mut tcp_connect = net::TcpStream::connect(&addr);
//    let write_stream = tcp_connect.map(|mut tcp_stream| {
//        let (sink, mut stream) = MessageStream.framed(tcp_stream).split();
//
//    });
//    //.flatten_stream();
//}

fn client(){
    let addr = "127.0.0.1:6666".parse::<std::net::SocketAddr>().unwrap();
    let mut tcp_connect = net::TcpStream::connect(&addr);
    let send = tcp_connect.map(|mut stream| {
        let (reader,mut writer)=stream.split();
        let _ = writer.write_all(&vec![1, 1][..]).unwrap();
        println!("already write");
        reader
    }).and_then(|mut stream|{
        let reader = io::read_to_end(stream, Vec::new())
            .and_then(|(_, bytes)| {
            println!("{:?}", bytes);
            Ok(())
        }).map_err(|e| println!("{:?}", e));
        tokio::spawn(reader);
        Ok(())
    }).map_err(|e| {
        println!("{:?}", e)
    });
    tokio::run(send);
}
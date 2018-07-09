extern crate tokio;
extern crate tokio_uds;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tempdir;

use bincode::{deserialize, serialize};

use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use tokio_uds::*;
use tokio::io;

use std::net::SocketAddr;

use std::time::{Duration, SystemTime};

use std::path::*;

// mod trash;
// use std::os::unix::net::UnixStream;

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

fn main() {
    client();
}

#[test]
fn server()
{
    let listener = network::create_tcp_listener("127.0.0.1:6666");
    // let listener = network::create_uds_listener("socket");
    let server = listener.incoming().for_each(move |stream| {
        let read_future = io::read_to_end(stream, Vec::new())
            .into_future()
            .and_then(|(_, bytes)| {
                let msg: Message = deserialize(&bytes).unwrap();
                println!("{:?}", msg);
                match msg {
                    Message::processMsg(process) => {
                        //

                    }
                    Message::seedMsg(seed) => {
                        //

                    }
                    Message::stateMsg(sate) => {
                        //
                    }
                    _ => {
                        println!("error");
                    }
                }
                Ok(())
            }).map_err(|e| println!("{:?}", e));
        tokio::spawn(read_future);
        Ok(())
    }).map_err(|e| println!("{:?}", e));
    tokio::run(server);
}


fn client()
{
    let mut stream = network::connect_tcp("127.0.0.1:6666");
    // let mut stream = network::connect_uds("socket");
    let msg = Message::processMsg(Process {});
    let one_sec = Duration::from_secs(10);
    let sys_time = SystemTime::now();
    let mut times = 0;
    loop {
        let _ = stream.write_all(&(serialize(&msg).unwrap())[..]);
        times += 1;
        if sys_time.elapsed().unwrap() >= one_sec { break; }
    }
    println!("{}", times);
//    let addr: SocketAddr = "127.0.0.1:6666".parse().unwrap();
//    let mut stream = TcpStream::connect(&addr);
//    let client = stream.into_future()
//        .and_then(|mut socket| {
//            let msg = Message::processMsg(Process {});
//            let _ = socket.write_all(&(serialize(&msg).unwrap())[..]);
//            Ok(())
//        }).map_err(|e| { println!("{:?}", e) });
//    tokio::run(client);
}


mod network {
    extern crate tokio;
    extern crate tokio_uds;
    extern crate tempdir;

    use tokio::prelude::*;
    use tokio::net::TcpListener;
    use tokio_uds::UnixListener;

    use std::net::TcpStream;
    use std::os::unix::net::UnixStream;
    use std::net::SocketAddr;
    use std::path::Path;

    pub fn create_tcp_listener(socket_addr: &str) -> TcpListener {
        let socket_addr = socket_addr.parse::<SocketAddr>().unwrap();
        return TcpListener::bind(&socket_addr).unwrap();
    }

    pub fn create_uds_listener(socket_name: &str) -> UnixListener {
        let socket_name = ["./", socket_name].join("");
        let path = Path::new(&socket_name);
        return UnixListener::bind(path).unwrap();
    }

    pub fn connect_tcp(socket_addr: &str) -> TcpStream {
        let addr: SocketAddr = socket_addr.parse().unwrap();
        return TcpStream::connect(&addr).unwrap();
    }

    pub fn connect_uds(socket_name: &str) -> UnixStream {
        let socket_name = ["./", socket_name].join("");
        let path = Path::new(&socket_name);
        return UnixStream::connect(path).unwrap();
    }
}
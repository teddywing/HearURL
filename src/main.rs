use std::net::UdpSocket;
use std::io;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn make_socket() -> io::Result<()> {
    let mut socket = try!(UdpSocket::bind("127.0.0.1:34254"));

    // read from the socket
    let mut buf = [0; 10];
    let (amt, src) = try!(socket.recv_from(&mut buf));

    // send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    try!(socket.send_to(buf, &src));

    Ok(())
}

fn open_stream() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut url = String::new();
                stream.read_to_string(&mut url)?;

                println!("{}", url);
            }
            Err(e) => {}
        }
    }

    Ok(())
}

fn main() {
    // make_socket();
    open_stream();
}

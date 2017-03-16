use std::net::UdpSocket;
use std::io;

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

fn main() {
    make_socket();
}

use std::io::{self, Write};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn open_stream() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut url = String::new();
                stream.read_to_string(&mut url)?;

                println!("{}", url);
            }
            Err(e) => {
                write!(io::stderr(), "{}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    open_stream();
}

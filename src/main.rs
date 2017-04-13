use std::io::{self, Write};
use std::io::prelude::*;
use std::net::TcpListener;
use std::process::Command;

fn open_stream() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut url = String::new();
                stream.read_to_string(&mut url)?;

                Command::new("open")
                    .arg("-a")
                    .arg("Opera")

                    // Trim the trailing newline, otherwise this doesn't
                    // work
                    .arg(&url.trim_right())
                    .spawn()?;
            }
            Err(e) => {
                write!(io::stderr(), "{}", e)?;
            }
        }
    }

    Ok(())
}

fn main() {
    open_stream().unwrap_or_else(|e| {
        writeln!(io::stderr(), "{}", e)
            .expect("Failed printing to stderr");
    });
}

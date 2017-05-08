extern crate url;

use url::Url;

use std::io::{self, Write};
use std::io::prelude::*;
use std::net::TcpListener;
use std::process::Command;

pub fn open_stream(browser: String, port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(
        format!("127.0.0.1:{}", port)
    )?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut url = String::new();
                match stream.read_to_string(&mut url) {
                    Ok(_) => {},
                    Err(e) => writeln!(io::stderr(), "{}", e)?,
                };

                match Url::parse(url.as_str()) {
                    Ok(url) => {
                        match Command::new("open")
                            .arg("-a")
                            .arg(&browser)
                            .arg(&url.as_str())
                            .spawn() {
                            Ok(_) => {},
                            Err(e) => writeln!(io::stderr(), "{}", e)?,
                        };
                    },
                    Err(e) => writeln!(io::stderr(), "{}", e)?,
                };
            }
            Err(e) => {
                writeln!(io::stderr(), "{}", e)?;
            }
        }
    }

    Ok(())
}

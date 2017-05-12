// Copyright (c) 2017 Teddy Wing
//
// This file is part of HearURL.
//
// HearURL is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// HearURL is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with HearURL. If not, see <http://www.gnu.org/licenses/>.

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

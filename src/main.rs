extern crate getopts;

use getopts::Options;

use std::env;
use std::io::{self, Write};
use std::io::prelude::*;
use std::net::TcpListener;
use std::process::Command;

const DEFAULT_PORT: u16 = 37705;

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

fn print_usage(opts: Options) {
    let brief = "Usage: hearurl -b BROWSER";
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("b", "browser", "set browser", "BROWSER");
    opts.optopt("p", "port", "set port number", "PORT");
    opts.optflag("h", "help", "print this help menu");

    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if opt_matches.opt_present("h") {
        print_usage(opts);
        return
    }

    let browser = match opt_matches.opt_str("b") {
        Some(b) => b,
        None => {
            print_usage(opts);
            return
        },
    };

    let port: u16 = match opt_matches.opt_str("p") {
        Some(p) => p.parse().unwrap(),
        None => DEFAULT_PORT,
    };

    open_stream().unwrap_or_else(|e| {
        writeln!(io::stderr(), "{}", e)
            .expect("Failed printing to stderr");
    });
}

extern crate getopts;

use getopts::Options;

use std::env;
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

fn print_usage(opts: Options) {
    let brief = "Usage: hearurl -b BROWSER";
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("b", "browser", "set browser", "BROWSER");
    opts.optflag("h", "help", "print this help menu");

    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if opt_matches.opt_present("h") {
        print_usage(opts);
        return
    }

    let browser = if opt_matches.opt_present("b") {
        opt_matches.opt_str("b")
    } else {
        print_usage(opts);
        return
    };

    open_stream().unwrap_or_else(|e| {
        writeln!(io::stderr(), "{}", e)
            .expect("Failed printing to stderr");
    });
}

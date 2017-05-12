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

extern crate getopts;

extern crate hearurl;

use getopts::Options;

use std::env;
use std::io::{self, Write};

const DEFAULT_PORT: u16 = 37705;

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
        Some(p) => p.parse().expect("Unable to parse specified port"),
        None => DEFAULT_PORT,
    };

    hearurl::open_stream(browser, port).unwrap_or_else(|e| {
        writeln!(io::stderr(), "{}", e)
            .expect("Failed printing to stderr");
    });
}

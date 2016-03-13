extern crate getopts;

use std::env;
use self::getopts::Options;

pub fn compile() -> (String, String, String) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.reqopt("r", "root", "The site root path", "PATH");
    opts.optopt("b", "bind", "Server address", "IP");
    opts.optopt("p", "port", "Port", "NUMBER");

    let matched_opts = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let root = matched_opts.opt_str("r").unwrap();
    let bind = match matched_opts.opt_str("b") {
        Some(x) => { x }
        None => { String::from("0.0.0.0") }
    };
    let port = match matched_opts.opt_str("p") {
        Some(x) => { x }
        None => { String::from("6767") }
    };
    (root, bind, port)
}

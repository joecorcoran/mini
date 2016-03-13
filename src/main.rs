#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate getopts;

use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;
use nickel_mustache::Render;
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use getopts::Options;

fn page_path(root: &str, name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(root);
    path.push("pages");
    path.push(name);
    path.set_extension("html");
    path
}

fn layout_path(root: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(root);
    path.push("layout");
    path.set_extension("mustache");
    path
}

fn match_opts() -> getopts::Matches {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("r", "root", "The site root path", "PATH");
    match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    }
}

fn main() {
    let mut server = Nickel::new();
    let matched_opts = match_opts();
    let root = matched_opts.opt_str("r").unwrap();

    server.get("/:page", middleware! { |request, response|
        let page = request.param("page").unwrap_or("missing");
        let path = page_path(&root, &page);
        if !path.is_file() {
            return response.error(StatusCode::NotFound, "Page not found");
        }
        let data: HashMap<&str, &str> = HashMap::new();
        return response.render_with_layout(path, layout_path(&root), &data)
    });

    server.listen("127.0.0.1:6767");
}

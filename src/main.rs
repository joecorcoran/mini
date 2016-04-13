#[macro_use] extern crate nickel;
extern crate nickel_mustache;

use nickel::{Nickel, HttpRouter, Mountable, StaticFilesHandler};
use nickel::status::StatusCode;
use nickel_mustache::Render;
use std::collections::HashMap;

mod sitepath;
mod opts;

fn main() {
    let mut server = Nickel::new();
    let (root, bind, port) = opts::compile();

    let root_copy = root.clone();
    server.get("/:page", middleware! { |request, response|
        let page = request.param("page").unwrap_or("missing");
        let path = sitepath::page_path(&root_copy, &page);
        if !path.is_file() {
            return response.error(StatusCode::NotFound, "Page not found");
        }
        let data: HashMap<&str, &str> = HashMap::new();
        return response.render_with_layout(path, sitepath::layout_path(&root_copy), &data)
    });

    server.mount("/static/", StaticFilesHandler::new(sitepath::static_path(&root)));

    let mut socket_addr = String::new();
    socket_addr.push_str(&bind);
    socket_addr.push_str(":");
    socket_addr.push_str(&port);
    server.listen(socket_addr.as_str());
}

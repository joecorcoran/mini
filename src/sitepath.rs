use std::path::PathBuf;

pub fn page_path(root: &str, name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(root);
    path.push("pages");
    path.push(name);
    path.set_extension("html");
    path
}

pub fn layout_path(root: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(root);
    path.push("layout");
    path.set_extension("mustache");
    path
}

pub fn static_path(root: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(root);
    path.push("static");
    path
}

extern crate toml_loader;
extern crate toml;

use toml_loader::Loader;
use std::path::Path;
use toml::Value;

fn pretty_print(val: &Value, indent: usize) {
    match val {
        &Value::Table(ref tbl) => {
            println!("{:2$}{}", "", "{", indent);
            tbl.iter().inspect(|&(k, v)| {
                println!("{:2$}{}", "", k, indent);
                pretty_print(v, indent + 2);
            }).count();
            println!("{:2$}{}", "", "}", indent);
        },
        &Value::Array(ref arr) => {
            println!("{:2$}{}", "", "[", indent);
            arr.iter().inspect(|&v| {
                pretty_print(v, indent + 2);
            }).count();
            println!("{:2$}{}", "", "]", indent);
        },
        v => println!("{:2$}{}", "", v, indent)
    }
}

fn main() {
    //let paths = vec![Path::new("/home/kerhong/Documents/Code/toml-loader/test.toml").to_path_buf()];
    //let toml = Loader::from_multiple_files(&paths).unwrap();
    let toml = Loader::from_file(Path::new("/home/kerhong/Documents/Code/toml-loader/test.toml")).unwrap();

    println!("{:?}", toml);
    println!("");

    pretty_print(&toml, 0);
    println!("");
    println!("");
    match toml.lookup("") {
        Some(v) => pretty_print(v, 0),
        None => println!("NOT FOUND")
    }
}

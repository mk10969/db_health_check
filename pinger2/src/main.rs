#[macro_use]
extern crate clap;

fn main() {
    println!("Hello, world!");

    let version = env!("CARGO_PKG_VERSION");
    println!("version: {}", &version);
    println!("version: {}", crate_version!()); //内部で、上記呼び出しているだけw

    let args = pinger2::args_parser::new(crate_version!()).get_matches();
}

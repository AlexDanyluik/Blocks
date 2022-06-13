extern crate blocks_lib;
use blocks_lib::parser::parse;

const CODE: &str = include_str!("../a.txt");

fn main() {
    println!("{:#?}", parse(CODE.to_owned()));
}

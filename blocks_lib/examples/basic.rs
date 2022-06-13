use blocks_lib::engine::Engine;

const CODE: &str = include_str!("example.blocks");

fn main() {
    let engine = Engine::from_str(CODE.to_owned());
    let output = engine.output();
    println!("{:?}", output);
}

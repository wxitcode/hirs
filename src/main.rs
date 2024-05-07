use std::env;

fn main() {
    println!("Hello, world!");
    println!("{}-{}", env::consts::OS, env::consts::ARCH);
}

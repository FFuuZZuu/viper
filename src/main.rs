use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    viper::compile::compile(args[1].clone());
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: lua [script]");
    }

    let lua_file: &str = &args[1];

    println!("{}", lua_file);
}

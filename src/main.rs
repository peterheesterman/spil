
use std::env;
use std::fs;

mod spil;

/*
 * Testing commands
  cargo run -- ../test-data/program.sp
  cargo run -- ../test-data/simple-expression.sp
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must pass at least a path to some source code.");
        return;
    }

    let source_file_path = args[1].clone();
    let source_code = fs::read_to_string(source_file_path)
        .expect("Something went wrong reading the file");

    spil::run(source_code);
}

use std::env;
use std::process;

fn run_file(file: &str) {
    println!("run file: {:?}", file);
}

fn run_prompt() {
    println!("run prompt");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = args.iter().skip(1).collect::<Vec<_>>();

    if params.len() > 1 {
        println!("Usage: rs-lisp [script]");
        process::exit(1);
    } else if params.len() == 1 {
        run_file(params[0]);
    } else {
        run_prompt();
    }
}

use std::env;
use std::process;

fn run (data: String) {
    println!("{}", data);
}

fn run_file(file: &str, had_error: bool) {
    let data = std::fs::read_to_string(file).unwrap();
    run(data);
    if had_error {
        process::exit(65);
    }
}

fn run_prompt() {
    println!("run prompt");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = args.iter().skip(1).collect::<Vec<_>>();

    let had_error = false;

    if params.len() > 1 {
        println!("Usage: rs-lisp [script]");
        process::exit(1);
    } else if params.len() == 1 {
        run_file(params[0], had_error);
    } else {
        run_prompt();
    }
}

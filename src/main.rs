mod shell;
use lite::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.iter().nth(1).unwrap_or(&String::new()).as_str() {
        "repl" | "-s" => shell::run().expect("something went wrong with rustyline"),
        "run" | "-r" if args.len() == 3 => run_file(&args[2]),
        "run" | "-r" => println!("'run' command takes one file argument."),
        "--help" | "-h" => command_line_help(),
        _ => {
            println!("Usage: lite-lang [path]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    let mut env = Environment::new();
    let file = std::fs::read_to_string(path);
    match file {
        Ok(input) => match program().parse((input, None)) {
            Ok(((_, _), vec_exp)) => {
                for exp in vec_exp {
                    env = match eval(exp, env) {
                        Ok((_, e)) => e,
                        Err((_, e)) => e,
                    };
                }
            }
            Err(e) => println!("{:#?}", e),
        },
        Err(_) => {
            println!("Failed to read file.");
            std::process::exit(74);
        }
    };
}

fn command_line_help() {
    println!(
        "lite lang ARGS:
verison 0.1.0

repl   | -s        : Runs the interactive Repl.
run    | -r [FILE] : Run takes a file and runs it.
--help | -h        : Display this help message.
"
    );
}

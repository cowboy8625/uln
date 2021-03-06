mod files;
mod interpreter;
mod parser;
mod shell;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap_or(&String::new()).as_str() {
        "repl" | "-s" => shell::run().expect("something went wrong with rustyline"),
        "run" | "-r" if args.len() == 3 => files::run(&args[2]),
        "run" | "-r" => println!("'run' command takes one file argument."),
        "--help" | "-h" => command_line_help(),
        _ => {
            println!("Usage: rlang [path]");
            std::process::exit(64);
        }
    }
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

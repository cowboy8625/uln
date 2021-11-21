// mod combinators;
// mod error;
// mod interpreter;
// mod node;
// mod parser;
// mod value;

// pub use combinators::{ParseResult, Parser};
// pub use interpreter::{eval, Environment};
// pub use node::{Node, Operator};
// pub use parser::program;
// pub use value::Value;
use lite::*;
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => shell(),
        2 => run_file(&args[1]),
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
            Err(_) => {}
        },
        Err(_) => {
            println!("Failed to read file.");
            std::process::exit(74);
        }
    };
}

pub fn shell() {
    print!("\x1b[2J\x1b[0;0H");
    let mut env = Environment::new();
    let mut block = String::new();
    let mut debug_output = false;
    let mut debug_node = false;
    let mut debug_input = false;
    let mut debug_env = false;
    loop {
        print!("\x1b[32m[IN]:\x1b[37m ");
        stdout().flush().expect("Flush Failed");
        std::io::stdin()
            .read_line(&mut block)
            .expect("Failed to read line");
        match block.trim() {
            ":exit" => break,
            ":clear" => print!("\x1b[2J\x1b[0;0H"),
            ":debug io" => debug_output = !debug_output,
            ":debug node" => debug_node = !debug_node,
            ":debug input" => debug_input = !debug_input,
            ":debug env" => debug_env = !debug_env,
            ":help" => help(),
            _ => match program().parse((block.clone().into(), None)) {
                Ok(((doc, _), vec_exp)) => {
                    for exp in vec_exp {
                        env = match eval(exp.clone(), env) {
                            Ok((v, e)) => {
                                if debug_env {
                                    println!("[ENV]: {:?}", e);
                                }
                                if debug_input {
                                    println!("[REMANDING INPUT]: {:?}", doc);
                                }
                                if debug_node {
                                    println!("[NODE]: {:?}", exp);
                                }
                                if Value::NONE != v {
                                    println!("[OUT]: {}", v);
                                    if debug_input {
                                        println!("[REMANDING INPUT]: {:?}", doc);
                                    }
                                    if debug_node {
                                        println!("[NODE]: {:?}", exp);
                                    }
                                }
                                e
                            }
                            Err((error, e)) => {
                                println!("[ERROR]: {}", error);
                                if debug_input {
                                    println!("[REMANDING INPUT]: {:?}", doc);
                                }
                                if debug_node {
                                    println!("[NODE]: {:?}", exp);
                                }
                                e
                            }
                        };
                    }
                }
                Err(e) => println!("\x1b[31m[ERROR]:\x1b[37m {:#?}", e),
            },
        }

        block.clear();
    }
}

fn help() {
    println!(
        "\x1b[1m\x1b[31m[HELP]:\x1b[37m \n{}",
        format!(
            "Shell Commands start with -> {red}:
    {green}:exit{reset} ---------> {cyan}exit program.
    {green}:help{reset} ---------> {cyan}Output this message.
    {green}:clear{reset} --------> {cyan}Clear shell screen.
    {green}:debug_io{reset} -----> {cyan}shows Value output, more useful for development.
    {green}:debug_node{reset} ---> {cyan}shows AST output, more useful for development.{reset}


    Language Syntax:
        hello = {green}\"Hello\"{reset}
        space = {green}\" \"{reset}
        world = {green}\"World\"{reset}
        {cyan}print{reset} hello + space + world {reset}{reset_font}
                         ",
            green = "\x1b[32m",
            reset = "\x1b[37m",
            cyan = "\x1b[36m",
            red = "\x1b[31m",
            reset_font = "\x1b[0m"
        )
    );
}

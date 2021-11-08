// program        → declaration* EOF ;
// declaration    → letDecl | statement ;
// statement      → exprStmt | printStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
//
// letDecl        → "let" IDENTIFIER ( "=" expression )? ";" ;
//
// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
use parser_combinator::*;
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
    let env = Environment::new();
    let file = std::fs::read_to_string(path);
    match file {
        Ok(input) => match program().parse((input, None)) {
            Ok((_, nodes)) => match eval(nodes, env) {
                Ok(_) => {}
                Err(e) => println!("\x1b[31m[OUT]:\x1b[37m {:#?}", e),
            },
            Err(e) => println!("\x1b[31m[ERROR]:\x1b[37m {:#?}", e),
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
            ":help" => help(),
            _ => match program().parse((block.trim().into(), None)) {
                Ok(((doc, _), nodes)) => {
                    if debug_node {
                        println!("\x1b[93m[NODES]:\x1b[95m {:#?}\x1b[37m", nodes);
                    }
                    if debug_output {
                        env = match eval(nodes, env) {
                            Ok((v, env)) => {
                                println!("\x1b[31m[OUT]:\x1b[37m {:#?}", v);
                                env
                            }
                            Err((e, env)) => {
                                println!("\x1b[31m[DEBUG_OUT]:\x1b[37m {:#?}", e);
                                env
                            }
                        };
                    } else {
                        env = match eval(nodes, env) {
                            Ok((v, env)) => {
                                match v {
                                    Value::NONE => {}
                                    v => println!("\x1b[31m[OUT]:\x1b[37m {}", v),
                                }
                                env
                            }
                            Err((e, env)) => {
                                println!("\x1b[31m[ERROR_OUT]:\x1b[37m {}", e);
                                env
                            }
                        };
                    }
                    if !doc.is_empty() {
                        println!("\x1b[31m[REMAINING]:\x1b[37m {:#?}", doc);
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

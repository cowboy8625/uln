// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" ;
use parser_combinator::*;
use std::io::{stdout, Write};

fn main() {
    shell();
}

pub fn shell() {
    print!("\x1b[2J\x1b[0;0H");
    let mut block = String::new();
    let mut debug = false;
    loop {
        print!("\x1b[32m[IN]:\x1b[37m ");
        stdout().flush().expect("Flush Failed");
        std::io::stdin()
            .read_line(&mut block)
            .expect("Failed to read line");
        match block.trim() {
            ":exit" => break,
            ":clear" => print!("\x1b[2J\x1b[0;0H"),
            ":debug" => debug = !debug,
            _ => match expression().parse(block.trim()) {
                Ok((doc, nodes)) => {
                    // println!("\x1b[93m[NODES]:\x1b[95m {:#?}\x1b[37m", nodes);
                    if debug {
                        match eval(nodes) {
                            Ok(v) => println!("\x1b[31m[OUT]:\x1b[37m {:#?}", v),
                            Err(e) => println!("\x1b[31m[OUT]:\x1b[37m {:#?}", e),
                        }
                    } else {
                        let value = match eval(nodes) {
                            Ok(v) => v.to_string(),
                            Err(e) => e.to_string(),
                        };
                        println!("\x1b[31m[OUT]:\x1b[37m {}", value);
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

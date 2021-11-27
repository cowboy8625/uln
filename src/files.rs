use crate::interpreter::interpreter;
use crate::parser;
pub fn run(path: &str) {
    let file = std::fs::read_to_string(path);
    match file {
        Ok(doc) => run_code(&doc),
        Err(_) => {
            println!("Failed to read file.");
            std::process::exit(74);
        }
    };
}

fn run_code(block: &str) {
    if block.is_empty() {
        return;
    }
    match parser::parser(block) {
        Ok((input, expr)) => {
            for cons in interpreter(expr) {
                if !input.is_empty() {
                    println!("[OUT]: {:?}", cons);
                    println!("[LEFTOVER]: {:?}", input);
                }
            }
        }
        Err(e) => println!("[ERROR]: {}", e),
    }
}

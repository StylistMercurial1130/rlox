mod cli;
mod interpreter;

use std::fs;
use interpreter::tokenizer;

fn main() {
    let test_code = fs::read_to_string("./test.rlox").expect("file not found");
    let mut tokenizer = tokenizer::Tokenizer::new(test_code);    
    if let Some(tokens) = tokenizer.generate_tokens() {
        tokens.iter().for_each(|token| {
            println!("{:?}",token.to_string());
        }); 
    }
}

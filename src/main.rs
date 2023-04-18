mod ast;
mod lexer;
mod parser;
mod token;

use reedline::{DefaultPrompt, Reedline, Signal};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;

const EXAMPLE_CODE: &'static str = r#"
actor hello_world {
    on startup {
        print "Hello, World!"
        die
    }
}
"#;

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let tokens: Vec<Token> = Lexer::new(&EXAMPLE_CODE).into_iter().collect();
                let parse = Parser::new(&tokens).parse().unwrap();
                println!("{:#?}", parse);

            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                break;
            }
            _ => {}
        }
    }

    println!("Hello, world!");
}

fn get_characters(input: &str, start: usize, count: usize) -> String {
    input
        .char_indices()
        .skip(start)
        .take(count)
        .map(|(_, c)| c)
        .collect()
}

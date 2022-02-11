use std::fs::read_to_string;
use clap::Parser;

/// Arguments handler.
pub mod args;
use args::{ Args, Options };

/// A front-end for the compiler.
/// Contains parser and tokenizer.
/// TODO: Semantic analysis and Type checking.
pub mod front;
use front::parser::parse;

/// A middle-end for the compiler.
/// Contains instructions generator.
pub mod middle;
use middle::gen::generate_instructions;

fn main() {
    let args = Args::parse();
    match args.options {
        Options::Compile { input, ast } => {
            let code = read_to_string(input).unwrap();
            let tree = parse(&code);
            match ast {
                true => for node in tree { println!("{:#?}", node) },
                false => {
                    let mut checked_tree = Vec::new();
                    for node in tree {
                        match node {
                            Ok(node) => checked_tree.push(node),
                            Err(err) => println!("{:?}", err),
                        }
                    };

                    let instructions = generate_instructions(checked_tree.into_iter());
                    for instruction in instructions {
                        println!("{:#?}", instruction);
                    }
                },
            }
        },
    }
}
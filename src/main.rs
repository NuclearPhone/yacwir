#![allow(dead_code)]
// ^ remove this later

use context::CompilerContextBuilder;
use parser::Parser;

mod context;
mod emitter;
mod ir;
mod lexer;
mod node;
mod parser;
mod token;
// mod x86_emitter;

fn main() {
  let mut args = std::env::args();

  if args.len() != 2 {
    println!("Expected an input file-name");
    std::process::exit(1);
  }

  let filename = args.nth(1).unwrap();

  let Ok(filedata) = std::fs::read_to_string(filename.clone()) else {
    println!("Failed to read file {filename}");
    std::process::exit(1);
  };

  let ctx = CompilerContextBuilder::new()
    .filedata(filedata)
    .verbose(true)
    .take();

  let ast = Parser::new(&ctx).unwrap().parse();

  println!("{:?}", ast);
}

#![allow(dead_code)]
// ^ remove this later

use context::CompilerContextBuilder;
use parser::Parser;

use crate::{
  diagnostic::{Diagnostic, DiagnosticLevel},
  emitter::Emitter,
  ir::IrEmitter,
  optimizers::optimize,
};

mod context;
mod diagnostic;
mod emitter;
mod ir;
mod lexer;
mod node;
mod optimizers;
mod parser;
mod token;
mod typecheck;
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
    .filedata(filedata.trim().into())
    .verbose(true)
    .take();

  let ast = Parser::new(&ctx).unwrap().parse().unwrap();

  println!("{:?}", ast.nodes);

  let ir_out = IrEmitter::new(&ctx, &ast).emit().unwrap();
  let ir_out = optimize(&ctx, ir_out);
  println!("{}", ir_out.funcs[0].instrs);

  ctx.push_diagnostic(Diagnostic {
    tokidx: 3,
    info: "test".to_owned(),
    level: DiagnosticLevel::Info,
  });

  // print any diagnostics
  for diagnostic in ctx.get_diagnostics().iter() {
    println!("{}", diagnostic.display(&ctx, &ast.toks));
  }

  // let optimized_ir = optimize(&ctx, ir_out);
}

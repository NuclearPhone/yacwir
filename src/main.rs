#![allow(dead_code)]
// ^ remove this later

use context::CompilerContextBuilder;
use parser::Parser;

use crate::{
  diagnostic::{Diagnostic, DiagnosticLevel},
  emitter::Emitter,
  emitters::{ir2c_emitter, x86_emitter::X86Emitter},
  ir::IrEmitter,
  optimizers::optimize,
  sema::TypeChecker,
};

mod context;
mod diagnostic;
mod emitter;
mod emitters;
mod ir;
mod lexer;
mod node;
mod optimizers;
mod parser;
mod sema;
mod token;

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

  let ir_out = IrEmitter::emit(&ctx, &ast).unwrap();
  let ir = TypeChecker::typecheck(&ctx, ir_out).unwrap();
  // let ir = optimize(&ctx, ir);
  println!("{}", ir.funcs[0]);

  // print any diagnostics
  for diagnostic in ctx.get_diagnostics().iter() {
    println!("{}", diagnostic.display(&ctx, &ast.toks));
  }

  // let asm = X86Emitter::emit(&ctx, &ir_out).unwrap();
  let asm = ir2c_emitter::Emitter::emit(&ctx, &ir).unwrap();
  println!("\n==== ASM OUTPUT ====\n{}", asm);
}

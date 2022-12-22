#![allow(dead_code)]
// ^ remove this later

use std::rc::Rc;

use context::CompilerContextBuilder;
use parser::Parser;

use crate::{
  ast2ir::IrEmitter, emitter::Emitter, emitters::ir2c_emitter, ir::IrFuncDisplay,
  optimizers::optimize, sema::SemaContext,
};

mod ast2ir;
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

  for (i, t) in ast.toks.iter().enumerate() {
    if i % 5 == 4 {
      println!();
    }
    print!("{} ", t.ty);
  }
  println!();

  println!("{:?}", ast.nodes);
  println!("FUNCS: {:?}", ast.funcs);

  let ir_out = IrEmitter::emit(&ast).unwrap();
  let ir = SemaContext::run(&ctx, ir_out);
  // let ir = optimize(&ctx, ir);

  for func in ir.funcs.iter() {
    println!("{}\n", IrFuncDisplay(&ctx, func));
  }

  // print any diagnostics
  for diagnostic in ctx.get_diagnostics().iter() {
    println!("{}", diagnostic.display(&ctx, &ast.toks));
  }

  // let asm = X86Emitter::emit(&ctx, &ir_out).unwrap();
  let asm = ir2c_emitter::Ir2CEmitterContext::emit(&ctx, &ast, ir).unwrap();
  println!("\n==== ASM OUTPUT ====\n{}", asm);
}

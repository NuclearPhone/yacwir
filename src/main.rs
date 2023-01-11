use context::CompilerContextBuilder;
use parser::Parser;

use crate::{
  optimizers::optimize,
  sema::{type_propogation::propogate, SemaContext},
};

mod ast2ir;
mod context;
mod diagnostic;
mod emitters;
mod ir;
mod lexer;
mod node;
mod optimizers;
mod parser;
mod sema;
mod token;

struct Arguments {
  filename: String,
}

fn parse_args() -> Result<Arguments, String> {
  let mut args = std::env::args();

  if args.len() != 2 {
    return Err("Expected an input filename".to_string());
  }
  args.next().unwrap();

  let filename = args.next().unwrap();

  Ok(Arguments { filename })
}

fn main() {
  let args = match parse_args() {
    Ok(args) => args,
    Err(e) => {
      println!("Error: {}", e);
      std::process::exit(1);
    }
  };

  let Ok(filedata) = std::fs::read_to_string(args.filename.clone()) else {
    println!("Failed to read file {}", args.filename);
    std::process::exit(1);
  };

  let ctx = CompilerContextBuilder::new()
    .filedata(filedata.trim().into())
    .verbose(true)
    .take();

  let ast = Parser::new(&ctx).unwrap().parse().unwrap();

  // for (i, t) in ast.toks.iter().enumerate() {
  //   if i % 5 == 4 {
  //     println!();
  //   }
  //   print!("{} ", t.ty);
  // }

  let mut sema_ctx = SemaContext::new(&ctx);

  let ir_out = ast2ir::emit(&ctx, &mut sema_ctx, &ast);
  let ir_out = propogate(&ctx, &sema_ctx, ir_out);
  // let ir_out = optimize(&ctx, &sema_ctx, ir_out);
  println!("{}", ir_out.display(&ctx));

  // let c_out = ir2c_emitter::emit(&ctx, ir_out);
  // println!("{}\n", c_out);

  // std::fs::write("./out.c", c_out).unwrap();

  // for func in ir.funcs.iter() {
  //   println!("{}\n", IrFuncDisplay(&ctx, func));
  // }

  // // print any diagnostics
  // for diagnostic in ctx.get_diagnostics().iter() {
  //   println!("{}", diagnostic.display(&ctx, &ast.toks));
  // }

  // // let asm = X86Emitter::emit(&ctx, &ir_out).unwrap();
  // let asm = ir2c_emitter::Ir2CEmitterContext::emit(&ctx, &ast, ir).unwrap();
  // println!("\n==== ASM OUTPUT ====\n{}", asm);
}

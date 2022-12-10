use crate::{context::CompilerContext, parser::Ast};

pub trait Emitter<'a> {
  type Input;
  type Output;

  fn emit(ctx: &'a CompilerContext, ast: &'a Ast, input: Self::Input) -> Self::Output;
}

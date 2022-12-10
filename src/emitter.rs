use crate::{context::CompilerContext, node::Node, parser::Ast};

pub trait Emitter<'a> {
  type Input;
  type Output;

  fn emit(ctx: &'a CompilerContext, ast: &'a Ast, input: Self::Input) -> Self::Output;
}

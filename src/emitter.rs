use crate::{context::CompilerContext, node::Node, parser::Ast};

pub trait Emitter<'a, 'b> {
  type Input;
  type Output;

  fn emit(ast: &'a Ast<'a>, input: &'b Self::Input) -> Self::Output;
}

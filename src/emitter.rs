use crate::{context::CompilerContext, node::Node, parser::Ast};

pub trait Emitter<'a> {
  type Input;
  type Output;

  fn emit(ast: &'a Ast, input: &'a Self::Input) -> Self::Output;
}

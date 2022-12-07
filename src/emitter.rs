use crate::{context::CompilerContext, node::Node};

pub trait Emitter<'a> {
  type Input;
  type Output;

  fn new(ctx: &'a CompilerContext, ast: &'a Self::Input) -> Self;
  fn emit(self) -> Self::Output;
}

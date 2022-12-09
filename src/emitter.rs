use crate::{context::CompilerContext, node::Node};

pub trait Emitter<'a, 'b> {
  type Input;
  type Output;

  fn emit(ctx: &'a CompilerContext, input: &'b Self::Input) -> Self::Output;
}

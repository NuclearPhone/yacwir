use crate::{context::CompilerContext, node::Node};

pub trait Emitter<'a> {
  fn new(ctx: &'a CompilerContext, ast: &'a Node) -> Self;
  fn emit(self) -> String;
}

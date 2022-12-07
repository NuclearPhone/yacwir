use crate::{context::CompilerContext, emitter::Emitter, node::Node};

struct X86Emitter<'a> {
  ast: &'a Node,
  ctx: &'a CompilerContext,

  buffer: String,
}

impl<'a> X86Emitter<'a> {
  fn emit_syntax(&mut self, node: &Node) {}

  fn inner_start_emit(&mut self) {
    self.buffer.push_str(".globl main\n.text\nmain:");
  }
}

impl<'a> Emitter<'a> for X86Emitter<'a> {
  fn new(ctx: &'a crate::context::CompilerContext, ast: &'a crate::node::Node) -> Self {
    Self {
      ast,
      ctx,
      buffer: String::new(),
    }
  }

  fn emit(mut self) -> String {
    self.inner_start_emit();
    self.buffer
  }
}

use std::{
  cell::{Cell, Ref, RefCell},
  rc::Rc,
};

use crate::{diagnostic::Diagnostic, optimizers::OptimizerFlags, token::Span};

/// context required for lexing, parsing, and emitting
/// contains all of the required information, and is passed around
/// to the various processi

#[derive(Default)]
pub struct CompilerContext {
  filedata: String,
  verbose: bool,

  optimizer_flags: OptimizerFlags,

  diagnostics: RefCell<Vec<Diagnostic>>,
}

impl CompilerContext {
  pub fn get_input_str(self: &Self) -> &str {
    &self.filedata
  }

  pub fn get_str_from_span(self: &Self, span: Span) -> &str {
    &self.filedata[span.start..span.end]
  }

  pub fn get_diagnostics(&self) -> Ref<Vec<Diagnostic>> {
    self.diagnostics.borrow()
  }

  pub fn get_optimizer_flags(self: &Self) -> OptimizerFlags {
    self.optimizer_flags.clone()
  }

  pub fn push_diagnostic(&self, diagnostic: Diagnostic) {
    self.diagnostics.borrow_mut().push(diagnostic);
  }
}

// builder struct for the compiler context
pub struct CompilerContextBuilder {
  ctx: CompilerContext,
}

impl CompilerContextBuilder {
  pub fn new() -> Self {
    Self {
      ctx: CompilerContext {
        verbose: false,
        ..Default::default()
      },
    }
  }

  pub fn filedata(mut self, input: String) -> Self {
    self.ctx.filedata = input;
    self
  }

  pub fn verbose(mut self, verbose: bool) -> Self {
    self.ctx.verbose = verbose;
    self
  }

  pub fn flags(mut self, flags: OptimizerFlags) -> Self {
    self.ctx.optimizer_flags = flags;
    self
  }

  pub fn take(self) -> CompilerContext {
    self.ctx
  }
}

use std::{cell::Cell, rc::Rc};

use crate::optimizers::OptimizerFlags;

/// context required for lexing, parsing, and emitting
/// contains all of the required information, and is passed around
/// to the various processi

pub enum DiagnosticLevel {
  Info,
  Warning,
  Error,
}

pub struct Diagnostic {
  // index into the token queue,
  // represents the root token where the diagnostic occured
  tokidx: usize,

  // relative danger level of the diagnostic
  level: DiagnosticLevel,
}

#[derive(Default)]
pub struct CompilerContext {
  filedata: String,
  verbose: bool,

  optimizer_flags: OptimizerFlags,

  diagnostics: Cell<Vec<Diagnostic>>,
}

impl CompilerContext {
  pub fn get_input_str(self: &Self) -> &str {
    &self.filedata
  }

  pub fn get_optimizer_flags(self: &Self) -> OptimizerFlags {
    self.optimizer_flags.clone()
  }

  pub fn push_diagnostic(&self, diagnostic: Diagnostic) {
    let mut diagnostics = self.diagnostics.take();
    diagnostics.push(diagnostic);
    self.diagnostics.set(diagnostics);
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

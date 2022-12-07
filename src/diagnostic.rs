use std::fmt::Display;

use crate::{context::CompilerContext, token::Token};

pub enum DiagnosticLevel {
  Info,
  Warning,
  Error,
}

pub struct Diagnostic {
  // index into the token queue,
  // represents the root token where the diagnostic occured
  pub tokidx: usize,

  // relative danger level of the diagnostic
  pub level: DiagnosticLevel,

  // string describing the diagnostic
  pub info: String,
}

impl Diagnostic {
  // convert the diagnostic to a printable string
  // requires context and tokens for lookup purposes
  pub fn display(&self, ctx: &CompilerContext, toks: &Vec<Token>) -> String {
    let str = ctx.get_input_str();

    // do some weird pointer arithmetic to get the location of the token in the main string
    let startpos = toks[self.tokidx].slice.as_ptr() as usize - str.as_ptr() as usize;

    // find the line position in input
    let lines = str[..startpos + 1].lines();
    let line = lines.clone().count();
    let line_data = lines.last().unwrap();

    let out = format!(
      "{}:    {}\n{}: | {}",
      line, line_data, self.level, self.info
    );
    out
  }
}

impl Display for DiagnosticLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Info => "Info",
      Self::Warning => "Warning",
      Self::Error => "Error",
    })
  }
}

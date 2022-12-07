use std::fmt::Display;

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

impl Display for DiagnosticLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Info => "Info",
      Self::Warning => "Warning",
      Self::Error => "Error",
    })
  }
}

impl Display for Diagnostic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // TODO: implement showing where the error is
    let out = format!("{}: {}", self.level, self.info);
    f.write_str(out.as_str())
  }
}

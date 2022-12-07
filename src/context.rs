/// context required for lexing, parsing, and emitting
/// contains all of the required information, and is passed around
/// to the various processi

#[derive(Default)]
pub struct CompilerContext {
  filedata: String,
  verbose: bool,
}

impl CompilerContext {
  pub fn get_input_str(self: &Self) -> &str {
    &self.filedata
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

  pub fn take(self) -> CompilerContext {
    self.ctx
  }
}

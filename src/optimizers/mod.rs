use crate::{context::CompilerContext, ir::IrUnit};

pub mod constant_folding;
pub mod dead_code;
pub mod defrag;

/*

// order of optimizations:

- constant folding

- dead code analysis
  dead code analysis should be last,
    as it allows all of the previous optimization passes to leave in
    dead code (e.g. removing the need of two constants) and then
    leaving the work to another function

*/

pub trait OptimizerPass<'a> {
  // apply an optimizing transform upon unit and return it
  fn transform(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit;
}

// a list of different optimizer passes that can be appled to
// the IR
#[derive(Clone)]
pub struct OptimizerFlags {
  const_folding: bool,
}

impl Default for OptimizerFlags {
  fn default() -> Self {
    Self {
      const_folding: true,
    }
  }
}

// eats @unit, and transforms it into a new IrUnit with optimizations
// applied to the code
pub fn optimize(ctx: &CompilerContext, unit: IrUnit) -> IrUnit {
  let flags = ctx.get_optimizer_flags();

  let unit = if flags.const_folding {
    constant_folding::Pass::transform(ctx, unit)
  } else {
    unit
  };

  unit
}

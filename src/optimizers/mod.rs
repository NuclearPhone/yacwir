use crate::{context::CompilerContext, ir::IrUnit};

pub mod constant_folding;
pub mod dead_code;

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

use crate::{context::CompilerContext, ir::IrUnit};

use super::OptimizerPass;

// performs dead code analysis on an entire unit
pub struct Pass<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> Pass<'a> {}

impl<'a> OptimizerPass<'a> for Pass<'a> {
  fn transform(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit {
    unimplemented!()
  }
}

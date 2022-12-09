// defragmenter

use crate::{context::CompilerContext, ir::IrUnit};

use super::OptimizerPass;

struct Pass<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> OptimizerPass<'a> for Pass<'a> {
  fn transform(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit {
    Self { ctx, unit }.defrag()
  }
}

impl<'a> Pass<'a> {
  fn defrag(self) -> IrUnit {
    unimplemented!()
  }
}

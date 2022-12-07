// ir typechecker,
// should be applied before any optimizations
// as to allow the optimizer passes to make assumptions
// and do things that it would otherwise have to check

// performs rudementary typechecking (e.g. erroring on incompatable types)
// also fleshes out auto-types

use crate::{context::CompilerContext, ir::IrUnit};

pub struct TypeChecker<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> TypeChecker<'a> {
  pub fn new(ctx: &'a CompilerContext, unit: IrUnit) -> Self {
    Self { ctx, unit }
  }

  pub fn typecheck_block(&self) {}
  pub fn typecheck_function(&self) {}

  pub fn typecheck(mut self) -> Result<IrUnit, (IrUnit, String)> {
    Ok(self.unit)
  }
}

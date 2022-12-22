use crate::{context::CompilerContext, ir::IrUnit};

use self::types::Types;

// mod binary_lowering;
// mod type_checking;
mod type_propogation;
mod types;

// the main typechecking construct
// manages all of the individual processes required
// does not handle the typechecking itself, instead
//   calls upon different processes
pub struct SemaContext<'a> {
  ctx: &'a CompilerContext,
  types: Types,
}

impl<'a> SemaContext<'a> {
  fn inner_run(self, mut unit: IrUnit) -> IrUnit {
    unit = type_propogation::propogate(&self, unit);
    unit
  }

  pub fn run(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit {
    Self {
      ctx,
      types: Types {},
    }
    .inner_run(unit)
  }
}

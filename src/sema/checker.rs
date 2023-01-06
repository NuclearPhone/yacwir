// the sema typechecker
// - checks that types are compatable throughout the entire program
// - converts implicit coercions into explicit cast instructions
// - converts comptime numerical types into their wordsize counterparts

use crate::{
  ir::{InstructionValue, IrUnit},
  token::Span,
};

use super::SemaContext;

pub fn check(sema: &SemaContext, unit: IrUnit) -> IrUnit {
  let mut new = IrUnit::default();

  for func in unit.funcs.iter() {}

  new.funcs = unit.funcs;

  new
}

struct FuncChecker<'a> {
  sema: &'a SemaContext,
  unit: &'a IrUnit,
}

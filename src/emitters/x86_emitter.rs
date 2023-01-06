use std::fmt::Display;

use crate::{
  context::CompilerContext,
  ir::{Instruction, InstructionValue, IrFunction, IrUnit},
  parser::Ast,
};

pub struct X86EmitterContext<'a> {
  ctx: &'a CompilerContext,
  unit: &'a IrUnit,

  buffer: String,
}

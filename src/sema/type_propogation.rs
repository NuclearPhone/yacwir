use crate::{
  context::CompilerContext,
  diagnostic::{Diagnostic, DiagnosticLevel},
  ir::{InstrIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit, Type},
  token::Span,
};

use super::SemaContext;

// - type-propogation does not rely on any sort
//   of instruction removal/addition to the instruction-map
//   thus it is safe to mutate the instruction map in-place without
//   having to adjust the block map
pub fn propogate<'a>(ctx: &'a CompilerContext, sema: &'a SemaContext, mut unit: IrUnit) -> IrUnit {
  // all reachable blocks start at the first block of every functions

  let func_clone = unit.funcs.clone();

  for func in func_clone {
    BlockTypePropogator {
      ctx,
      sema,
      span: unit.blocks[func.block as usize],
      unit: &mut unit,
    }
    .propogate();
  }

  unit
}

// type-lowering construct for a single block
// recursively creates a new block propogator for every
// block that branches off from the current one
// stops execution when running into a HALT/Return block
struct BlockTypePropogator<'a> {
  ctx: &'a CompilerContext,
  sema: &'a SemaContext,

  // the unit that which this block is associated with
  unit: &'a mut IrUnit,

  // the associated span of the block
  span: Span,
}

impl<'a> BlockTypePropogator<'a> {
  fn propogate(mut self) {
    for i in self.span.start..self.span.end {
      self.propogate_instruction(i as u32);
    }
  }

  fn propogate_instruction(&mut self, instridx: InstrIdx) {
    let instr = &self.unit.instructions[instridx as usize];

    let tok = instr.tok;

    let out = match instr.val {
      // constants have their types generated at emission,
      // thus nothing has to happen
      InstructionValue::ConstInteger(_) | InstructionValue::ConstFloat(_) => instr.clone(),

      InstructionValue::Add(l, r)
      | InstructionValue::Subtract(l, r)
      | InstructionValue::Multiply(l, r)
      | InstructionValue::Divide(l, r) => {
        // instructions can only reference instructions that come before them,
        // thus these instructions are guaranteed to be typed
        let l_ty = self.unit.instructions[l as usize].ty;
        let r_ty = self.unit.instructions[r as usize].ty;

        let ty = if !self.sema.types.binary_compatable_types(l_ty, r_ty) {
          self.ctx.push_diagnostic(Diagnostic {
            info: format!(
              "Invalid binary operation types in add operator: {} and {}",
              l_ty, r_ty
            ),

            level: DiagnosticLevel::Error,

            tokidx: instr.tok,
          });

          Type::Invalid
        } else if !self.sema.types.coerce_type(l_ty, r_ty) {
          self.ctx.push_diagnostic(Diagnostic {
            info: format!(
              "Unable to coerce the left side operand of a binary operation to the type of the right side operand"
            ),
            level: DiagnosticLevel::Error,
            tokidx: instr.tok,
          });

          Type::Invalid
        } else {
          r_ty
        };

        Instruction {
          val: instr.val.clone(),
          ty,
          tok,
        }
      }

      InstructionValue::Return(ridx) => {
        let ty = self.unit.instructions[ridx as usize].ty;

        Instruction {
          val: instr.val.clone(),
          ty,
          tok,
        }
      }

      _ => unimplemented!(),
    };

    self.unit.instructions[instridx as usize] = out;
  }
}

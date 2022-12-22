use crate::{
  diagnostic::{Diagnostic, DiagnosticLevel},
  ir::{InstrIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit, Type},
};

use super::SemaContext;

pub fn propogate<'a>(sema: &'a SemaContext<'a>, unit: IrUnit) -> IrUnit {
  let mut funcs = vec![];

  for func in unit.funcs.iter() {
    funcs.push(FunctionTypePropogator::propogate(sema, func));
  }

  IrUnit { funcs }
}

// type-lowering construct for a single function
// contains a reference to the global typechecking context
struct FunctionTypePropogator<'a> {
  sema: &'a SemaContext<'a>,
  function: &'a IrFunction,
  out_buffer: Vec<Instruction>,
}

impl<'a> FunctionTypePropogator<'a> {
  fn propogate_instruction(&mut self, instridx: InstrIdx) {
    let instr = &self.function.instrs.0[instridx];

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
        let l_ty = self.out_buffer[l].ty;
        let r_ty = self.out_buffer[r].ty;

        let ty = if !self.sema.types.binary_compatable_types(l_ty, r_ty) {
          self.sema.ctx.push_diagnostic(Diagnostic {
            info: format!(
              "Invalid binary operation types in add operator: {} and {}",
              l_ty, r_ty
            ),

            level: DiagnosticLevel::Error,

            tokidx: instr.tok,
          });

          Type::Invalid
        } else if !self.sema.types.coerce_type(l_ty, r_ty) {
          self.sema.ctx.push_diagnostic(Diagnostic {
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
        let ty = self.out_buffer[ridx].ty;

        Instruction {
          val: instr.val.clone(),
          ty,
          tok,
        }
      }

      _ => unimplemented!(),
    };

    self.out_buffer.push(out);
  }

  fn inner_propogate(mut self) -> Self {
    for instr in 0..self.function.instrs.0.len() {
      self.propogate_instruction(instr)
    }
    self
  }

  pub fn propogate(typechecker: &'a SemaContext<'a>, function: &'a IrFunction) -> IrFunction {
    IrFunction {
      name: function.name,
      instrs: IrBlock(
        Self {
          sema: typechecker,
          function,
          out_buffer: vec![],
        }
        .inner_propogate()
        .out_buffer,
      ),
    }
  }
}

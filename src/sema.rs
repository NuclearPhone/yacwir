// ir typechecker,
// should be applied before any optimizations
// as to allow the optimizer passes to make assumptions
// and do things that it would otherwise have to check

// performs rudementary typechecking (e.g. erroring on incompatable types)
// also fleshes out auto-types

use crate::{
  context::CompilerContext,
  diagnostic::{Diagnostic, DiagnosticLevel},
  ir::{InstrIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit, Type},
};

pub struct TypeChecker<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> TypeChecker<'a> {
  // performs type coercion along with valid-typechecking
  fn compat_types(&self, left: Type, right: Type) -> Option<Type> {
    match (left, right) {
      (Type::Integer, Type::Integer) => Some(Type::Integer),
      (Type::Floating, Type::Floating) => Some(Type::Floating),
      _ => None,
    }
  }

  fn inner_typecheck(self) -> Result<IrUnit, String> {
    let mut funcs = vec![];

    for func in self.unit.funcs.iter() {
      funcs.push(FunctionSemaContext::typecheck(&self, func));
    }

    Ok(IrUnit { funcs })
  }

  pub fn typecheck(ctx: &'a CompilerContext, unit: IrUnit) -> Result<IrUnit, String> {
    Self { ctx, unit }.inner_typecheck()
  }
}

/*

// LIST OF THINGS TODO

- context dependent typechecking
  e.g. type x can be of type y because user has defined it


*/

// typechecker for a single function
// contains a reference to the global typechecker context
// also contains a reference to the function that is being typechecked
struct FunctionSemaContext<'a, 'b> {
  typechecker: &'b TypeChecker<'a>,
  function: &'b IrFunction,
  out_buffer: Vec<Instruction>,
}

impl<'a, 'b> FunctionSemaContext<'a, 'b> {
  fn typecheck_instruction(&mut self, instridx: InstrIdx) {
    let instr = &self.function.instrs.0[instridx];

    let tok = instr.tok;

    let out = match instr.val {
      // constants have their types generated at emission,
      // thus nothing has to happen
      InstructionValue::ConstInteger(_) | InstructionValue::ConstFloat(_) => instr.clone(),

      InstructionValue::Add(l, r) => {
        // instructions can only reference instructions that come before them,
        // thus these instructions are guaranteed to be typed
        let l_ty = self.out_buffer[l].ty;
        let r_ty = self.out_buffer[r].ty;

        let ty = self
          .typechecker
          .compat_types(l_ty, r_ty)
          .or_else(|| {
            self.typechecker.ctx.push_diagnostic(Diagnostic {
              info: format!(
                "Invalid binary operation types in add operator: {} and {}",
                l_ty, r_ty
              ),

              level: DiagnosticLevel::Error,

              tokidx: instr.tok,
            });

            Some(Type::Invalid)
          })
          .unwrap();

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

  fn inner_context(mut self) -> IrFunction {
    for idx in 0..self.function.instrs.0.len() {
      self.typecheck_instruction(idx);
    }

    IrFunction {
      name: self.function.name.clone(),
      instrs: IrBlock(self.out_buffer),
    }
  }

  pub fn typecheck(typechecker: &'b TypeChecker<'a>, function: &'b IrFunction) -> IrFunction {
    Self {
      typechecker,
      function,
      out_buffer: vec![],
    }
    .inner_context()
  }
}

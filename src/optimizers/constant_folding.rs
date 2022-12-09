use crate::{
  context::CompilerContext,
  ir::{Instruction, InstructionValue, IrBlock, IrFunction, IrUnit},
};

use super::OptimizerPass;

pub struct Pass<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> OptimizerPass<'a> for Pass<'a> {
  fn transform(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit {
    Self { ctx, unit }.inner_transform()
  }
}

impl<'a> Pass<'a> {
  // constants that are optimized away are still left in
  // the irblock, as they will be removed in dead code analysis
  fn transform_block(&self, from_block: &IrBlock) -> IrBlock {
    let mut to_block: Vec<Instruction> = vec![];

    for instr in from_block.0.iter() {
      match &instr.val {
        bin @ InstructionValue::Add(left, right)
        | bin @ InstructionValue::Subtract(left, right)
        | bin @ InstructionValue::Multiply(left, right)
        | bin @ InstructionValue::Divide(left, right) => {
          // we need to read from to_block,
          // as we are depending on previous optimizations
          // e.g. nested binary operations that can reduce to a constant
          let lval = to_block.get(*left).unwrap();
          let rval = to_block.get(*right).unwrap();

          match (&lval.val, &rval.val) {
            (InstructionValue::ConstInteger(li), InstructionValue::ConstInteger(ri)) => to_block
              .push(Instruction {
                tok: instr.tok,
                ty: instr.ty,
                val: InstructionValue::ConstInteger(match bin {
                  InstructionValue::Add(..) => li + ri,
                  InstructionValue::Subtract(..) => li - ri,
                  InstructionValue::Multiply(..) => li * ri,
                  InstructionValue::Divide(..) => li / ri,
                  _ => unreachable!(),
                }),
              }),

            (InstructionValue::ConstFloat(li), InstructionValue::ConstFloat(ri)) => {
              to_block.push(Instruction {
                tok: instr.tok,
                ty: instr.ty,
                val: InstructionValue::ConstFloat(match bin {
                  InstructionValue::Add(..) => li + ri,
                  InstructionValue::Subtract(..) => li - ri,
                  InstructionValue::Multiply(..) => li * ri,
                  InstructionValue::Divide(..) => li / ri,
                  _ => unreachable!(),
                }),
              })
            }

            // if neither are constants, we cant do anything, just return
            _ => break,
          }
        }

        _ => to_block.push(instr.clone()),
      }
    }

    IrBlock(to_block)
  }

  fn transform_function(&self, func: &IrFunction) -> IrFunction {
    IrFunction {
      name: func.name.to_owned(),
      instrs: self.transform_block(&func.instrs),
    }
  }

  fn inner_transform(self) -> IrUnit {
    let mut funcs = vec![];

    for func in self.unit.funcs.iter() {
      funcs.push(self.transform_function(func));
    }

    IrUnit { funcs }
  }
}

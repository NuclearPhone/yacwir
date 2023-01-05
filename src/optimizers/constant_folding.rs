use crate::{
  context::CompilerContext,
  ir::{BlockIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit},
  sema::SemaContext,
  token::Span,
};

// this pass does not add nor remove instructions from the unit
// thus it is safe to mutate instructions in place without
// updating the blocks

// constants that are optimized away are still left in
// the instruction map, as they are deleted in the dead-code
// analysis step

pub fn fold(sema: &SemaContext, mut unit: IrUnit) -> IrUnit {
  for function in unit.funcs.clone() {
    fold_block(sema, &mut unit, function.block);
  }
  unit
}

fn fold_block(sema: &SemaContext, unit: &mut IrUnit, blockidx: BlockIdx) {
  let span = unit.blocks[blockidx as usize];

  for instridx in span.start..span.end {
    let instr = &unit.instructions[instridx];

    match &instr.val {
      bin @ InstructionValue::Add(left, right)
      | bin @ InstructionValue::Subtract(left, right)
      | bin @ InstructionValue::Multiply(left, right)
      | bin @ InstructionValue::Divide(left, right) => {
        // we need to read from to_block,
        // as we are depending on previous optimizations
        // e.g. nested binary operations that can reduce to a constant
        let lval = &unit.instructions[*left as usize];
        let rval = &unit.instructions[*right as usize];

        let replacement = match (&lval.val, &rval.val) {
          (InstructionValue::ConstInteger(li), InstructionValue::ConstInteger(ri)) => Instruction {
            tok: instr.tok,
            ty: instr.ty,
            val: InstructionValue::ConstInteger(match bin {
              InstructionValue::Add(..) => li + ri,
              InstructionValue::Subtract(..) => li - ri,
              InstructionValue::Multiply(..) => li * ri,
              InstructionValue::Divide(..) => li / ri,
              _ => unreachable!(),
            }),
          },

          (InstructionValue::ConstFloat(li), InstructionValue::ConstFloat(ri)) => Instruction {
            tok: instr.tok,
            ty: instr.ty,
            val: InstructionValue::ConstFloat(match bin {
              InstructionValue::Add(..) => li + ri,
              InstructionValue::Subtract(..) => li - ri,
              InstructionValue::Multiply(..) => li * ri,
              InstructionValue::Divide(..) => li / ri,
              _ => unreachable!(),
            }),
          },

          // if neither are constants, we cant do anything, just return
          _ => continue,
        };

        unit.instructions[instridx] = replacement;
      }

      // these are implicitly typed
      InstructionValue::Return(_)
      | InstructionValue::ConstFloat(_)
      | InstructionValue::ConstInteger(_) => (),

      _ => unimplemented!(),
    }
  }
}

use std::fmt::Display;

use crate::{
  context::CompilerContext,
  emitter::Emitter,
  ir::{InstrIdx, InstructionValue, IrFunction, IrUnit},
  node::Node,
};

pub struct X86Emitter<'a, 'b> {
  ctx: &'a CompilerContext,
  unit: &'b IrUnit,

  buffer: String,
}

impl<'a, 'b> Emitter<'a, 'b> for X86Emitter<'a, 'b> {
  type Input = IrUnit;
  type Output = Result<String, String>;

  fn emit(ctx: &'a CompilerContext, unit: &'b IrUnit) -> Self::Output {
    Self {
      ctx,
      unit,
      buffer: String::new(),
    }
    .inner_start_emit()
  }
}

// "const" register : %rbx

impl<'a, 'b> X86Emitter<'a, 'b> {
  fn emit_const<T>(&mut self, node: T) -> Result<String, String>
  where
    T: Display,
  {
    Ok(format!("  movq ${}, %rbx", node))
  }

  fn emit_instr(&mut self, instr: &InstructionValue) -> Result<(), String> {
    let out = match instr {
      InstructionValue::ConstInteger(i) => self.emit_const(i),
      InstructionValue::ConstFloat(f) => self.emit_const(f),

      _ => unimplemented!(),
    }?;

    self.buffer.push_str(out.as_str());
    Ok(())
  }

  fn emit_function(&mut self, func: &IrFunction) -> Result<(), String> {
    let block = &func.instrs;

    let prelude = format!(".globl {}\n{}:\n", func.name, func.name);
    self.buffer.push_str(prelude.as_str());

    for i in block.0.iter() {
      self.emit_instr(&i.val)?;
    }

    Ok(())
  }

  fn inner_start_emit(mut self) -> Result<String, String> {
    for func in self.unit.funcs.iter() {
      self.emit_function(func)?;
    }

    Ok(self.buffer)
  }
}

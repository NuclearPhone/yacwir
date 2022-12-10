use std::{fmt::Display, rc::Rc};

use crate::{
  context::CompilerContext,
  emitter::Emitter,
  ir::{InstrIdx, InstructionValue, IrFunction, IrUnit},
  node::Node,
  parser::Ast,
};

pub struct X86Emitter<'a> {
  ctx: &'a CompilerContext,
  ast: &'a Ast,
  unit: &'a IrUnit,

  buffer: String,
}

impl<'a> Emitter<'a> for X86Emitter<'a> {
  type Input = &'a IrUnit;
  type Output = Result<String, String>;

  fn emit(ctx: &'a CompilerContext, ast: &'a Ast, unit: Self::Input) -> Self::Output {
    Self {
      ctx,
      ast,
      unit,
      buffer: String::new(),
    }
    .inner_start_emit()
  }
}

// "const" register : %rbx

impl<'a> X86Emitter<'a> {
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

    let prelude = format!(
      ".globl {}\n{}:\n",
      self.ctx.get_str_from_span(func.name),
      self.ctx.get_str_from_span(func.name)
    );

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

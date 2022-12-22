use std::fmt::Display;

use crate::{
  context::CompilerContext,
  emitter::Emitter,
  ir::{Instruction, InstructionValue, IrFunction, IrUnit},
  parser::Ast,
};

pub struct X86EmitterContext<'a> {
  ctx: &'a CompilerContext,
  ast: &'a Ast,
  unit: IrUnit,

  buffer: String,
}

impl<'a> Emitter<'a> for X86EmitterContext<'a> {
  type Input = IrUnit;
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

impl<'a> X86EmitterContext<'a> {
  fn inner_start_emit(mut self) -> Result<String, String> {
    for func in self.unit.funcs.iter() {
      let out = FunctionEmitter::emit(&self, &func);
    }

    Ok(self.buffer)
  }
}

struct FunctionEmitter<'a> {
  sema: &'a X86EmitterContext<'a>,
  func: &'a IrFunction,

  out_buffer: String,
}

impl<'a> FunctionEmitter<'a> {
  fn emit(ctx: &'a X86EmitterContext, func: &'a IrFunction) -> String {
    Self {
      sema: ctx,
      func,
      out_buffer: String::new(),
    }
    .inner_emit()
    .out_buffer
  }

  fn emit_instruction(&mut self, instr: &Instruction) {
    // let out = match instr {
    //   InstructionValue::ConstInteger(i) => self.emit_const(i),
    //   InstructionValue::ConstFloat(f) => self.emit_const(f),

    //   _ => unimplemented!(),
    // }?;

    // self.out_buffer.push_str(out.as_str());
  }

  fn inner_emit(mut self) -> Self {
    let prelude = format!(
      ".globl {}\n{}:\n",
      self.sema.ctx.get_str_from_span(self.func.name),
      self.sema.ctx.get_str_from_span(self.func.name)
    );

    self.out_buffer.push_str(prelude.as_str());

    for i in self.func.instrs.0.iter() {
      self.emit_instruction(i);
    }
    self
  }
}

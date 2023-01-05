use std::rc::Rc;

use crate::{
  context::CompilerContext,
  ir::{BlockIdx, FuncIdx, InstrIdx, InstructionValue, IrFunction, IrUnit, Type},
  parser::Ast,
  token::Span,
};

// TODO: mark which variables are actually reused and where,
//     and then when they are no longer to be used, reuse their stack space

pub struct IR2CEmitter<'a> {
  ctx: &'a CompilerContext,
  unit: &'a IrUnit,
  buffer: String,
}

pub fn emit(ctx: &CompilerContext, unit: IrUnit) -> String {
  IR2CEmitter {
    ctx,
    unit: &unit,
    buffer: String::new(),
  }
  .emit()
}

impl<'a> IR2CEmitter<'a> {
  // renders a type and pushes it to the end of the buffer
  // does not render a space after the type
  fn render_type(&mut self, ty: Type) {
    self.buffer.push_str(match ty {
      Type::Undecided | Type::Invalid => panic!(),

      Type::Floating => "double",
      Type::Integer => "long long",
      Type::Moot => "void",
    });
  }

  fn render_comment(&mut self, comment: &str) {
    let spaces = comment.split_whitespace();

    let mut span = Span { start: 0, end: 0 };

    for word in spaces {
      span.end += word.len() + 1;
      if span.end - span.start > 80 {
        self.buffer += "// ";
        self.buffer += &comment[span.start..span.end - 1];
        self.buffer += "\n";
        span.start = span.end;
      }
    }

    self.buffer += "// ";
    self.buffer += &comment[span.start..span.end - 1];
    self.buffer += "\n";
  }

  fn render_instr(&mut self, instridx: InstrIdx) {
    self.buffer += "\t";
    match self.unit.instructions[instridx as usize].val {
      InstructionValue::ConstInteger(i) => {
        self.render_type(Type::Integer);
        self.buffer += format!(" _{instridx} = {i};\n").as_str();
      }

      InstructionValue::ConstFloat(f) => {
        self.render_type(Type::Floating);
        self.buffer += format!(" _{instridx} = {f:?}f;\n").as_str();
      }

      InstructionValue::Return(idx) => {
        self.buffer += format!("return _{idx};\n").as_str();
      }

      _ => todo!(),
    }
  }

  fn render_block(&mut self, blockidx: BlockIdx) {
    let block = self.unit.blocks[blockidx as usize];

    self.buffer += format!("BLOCK{blockidx}:;\n").as_str();

    for idx in block.start..block.end {
      self.render_instr(idx as u32);
    }
  }

  fn render_function(&mut self, funidx: FuncIdx) {
    let function = &self.unit.funcs[funidx as usize];

    self.render_prototype(funidx);
    self.buffer += "{\n";

    self.render_block(function.block);

    self.buffer += "}\n\n";
  }

  // renders a prototype into the buffer, without an ending semicolon
  fn render_prototype(&mut self, funidx: FuncIdx) {
    let function = &self.unit.funcs[funidx as usize];

    // if this is main
    if funidx == 0 {
      self.buffer += "extern int main(int argc, char** argv)";
    } else {
      self.render_type(function.ret_type);
      self.buffer += " ";
      self.buffer += self.ctx.get_str_from_span(function.name);
      self.buffer += "()";
    }
  }

  fn render_prelude(&mut self) {
    self.render_comment("prelude start");

    self.render_comment("includes");
    self.buffer += "#include<stdio.h>\n#include<stdlib.h>\n\n";

    self.render_comment("prototypes");
    for func in 0..self.unit.funcs.len() {
      self.render_prototype(func as u32);
      self.buffer += ";\n";
    }
    self.buffer += "\n";

    self.render_comment("prelude end\n\n");
  }

  fn emit(mut self) -> String {
    self.render_prelude();

    for func in 0..self.unit.funcs.len() {
      self.render_function(func as u32);
    }

    self.buffer
  }
}

use crate::{
  context::CompilerContext,
  ir::{InstructionValue, IrFunction, IrUnit, Type},
  parser::Ast,
};

pub struct Emitter<'a> {
  ctx: &'a CompilerContext,
  ast: &'a Ast,
  unit: &'a IrUnit,
}

impl<'a> crate::emitter::Emitter<'a> for Emitter<'a> {
  type Input = &'a IrUnit;
  type Output = Result<String, String>;

  fn emit(ctx: &'a CompilerContext, ast: &'a Ast, unit: Self::Input) -> Self::Output {
    Self { ctx, ast, unit }.inner_emit()
  }
}

impl<'a> Emitter<'a> {
  fn emit_type(&self, ty: Type) -> String {
    match ty {
      Type::Floating => "double".to_string(),
      Type::Integer => "long long".to_string(),

      Type::Undecided | Type::Invalid => "ran into invalid types in typechecker".to_string(),
    }
  }

  fn emit_instruction(
    &self,
    buffer: &mut String,
    function: &IrFunction,
    instridx: usize,
  ) -> Result<(), String> {
    let instr = &function.instrs.0[instridx];

    let expr = match instr.val {
      InstructionValue::ConstInteger(i) => {
        format!("long long TEMP{} = (long long){};", instridx, i)
      }
      InstructionValue::ConstFloat(f) => format!("double TEMP{} = (double){}f;", instridx, f),

      InstructionValue::Add(l, r) => {
        self.emit_instruction(buffer, function, l)?;
        self.emit_instruction(buffer, function, r)?;
        format!(
          "{} TEMP{} = TEMP{} + TEMP{};",
          self.emit_type(instr.ty),
          instridx,
          l,
          r
        )
      }

      InstructionValue::Return(i) => format!("return TEMP{};", i),

      _ => unimplemented!(),
    };

    buffer.push_str(&expr);
    buffer.push('\n');

    Ok(())
  }

  fn emit_function(&self, function: &IrFunction) -> Result<String, String> {
    let mut buf = format!("int {}() {{\n", self.ctx.get_str_from_span(function.name));

    for idx in 0..function.instrs.0.len() {
      self.emit_instruction(&mut buf, function, idx)?;
    }

    buf.push('}');
    Ok(buf)
  }

  fn inner_emit(self) -> Result<String, String> {
    let mut file_buf = String::new();

    for func in self.unit.funcs.iter() {
      file_buf.push_str(&self.emit_function(func)?);
    }

    Ok(file_buf)
  }
}

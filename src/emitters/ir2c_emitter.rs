use std::rc::Rc;

use crate::{
  context::CompilerContext,
  ir::{InstructionValue, IrFunction, IrUnit, Type},
  parser::Ast,
};

pub struct Ir2CEmitterContext<'a> {
  ctx: &'a CompilerContext,
  ast: &'a Ast,
  unit: IrUnit,
}

struct FunctionEmitter<'a> {
  emitter: &'a Ir2CEmitterContext<'a>,
  function: &'a IrFunction,
}

impl<'a> crate::emitter::Emitter<'a> for Ir2CEmitterContext<'a> {
  type Input = IrUnit;
  type Output = Result<String, String>;

  fn emit(ctx: &'a CompilerContext, ast: &'a Ast, unit: Self::Input) -> Self::Output {
    Self { ctx, ast, unit }.inner_emit()
  }
}

impl<'a> Ir2CEmitterContext<'a> {
  fn emit_type(&self, ty: Type) -> String {
    match ty {
      Type::Floating => "double".to_string(),
      Type::Integer => "long long".to_string(),
      Type::Moot => "void".to_string(),

      Type::Undecided | Type::Invalid => "ran into invalid types in typechecker".to_string(),
    }
  }

  // TODO:
  //   implement custom add/sub/mul/div binary functions
  //   for each type

  fn generate_binary_functions(&self, buffer: &mut String, ty: Type) {
    match ty {
      Type::Undecided => panic!("ran into a non-propogated type in ir2c"),
      Type::Invalid => panic!("ran into invalid type in ir2c"),

      Type::Floating => buffer.push_str(
        "
            static inline double wolnir_double_add(double left, double right) {{
              return left + right;
            }}
          ",
      ),

      Type::Integer => buffer.push_str(
        "
            static inline long long wolnir_long_long_add(double left, double right) {{
              return left + right;
            }}
          ",
      ),

      // can not generate binary operation for moot
      Type::Moot => {}
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
    let name_str = self.ctx.get_str_from_span(function.name);
    let linkage = if name_str == "main" {
      "extern"
    } else {
      "static"
    };

    let mut buf = format!(
      "{linkage} int {name}() {{\n",
      linkage = linkage,
      name = name_str
    );

    for idx in 0..function.instrs.0.len() {
      self.emit_instruction(&mut buf, function, idx)?;
    }

    buf.push_str("}\n\n");
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

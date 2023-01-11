use super::ir::{Function, Instruction, Register, Unit, ValLoc};

pub fn emit(unit: Unit) -> String {
  let mut out = String::new();

  for fun in unit.funcs.iter() {
    out += emit_function(fun).as_str();
  }

  out
}

fn emit_function(func: &Function) -> String {
  let mut out = String::new();

  for instr in func.instrs.iter() {
    emit_instruction(&mut out, *instr);
  }

  out
}

fn emit_location(buffer: &mut String, loc: ValLoc) {
  let out = match loc {
    ValLoc::Register(reg) => match reg {
      Register::RAX => "%rax",
      Register::RBX => "%RBX",
      Register::RCX => "%RCX",
      Register::RDX => "%RDX",
    }
    .to_owned(),

    ValLoc::Memory(loc) => loc.to_string(),
    ValLoc::Float => todo!(),
  };

  *buffer += out.as_str();
}

fn emit_instruction(buffer: &mut String, instr: Instruction) {
  match instr {
    Instruction::Push(_) => todo!(),
    Instruction::Pop(_) => todo!(),
    Instruction::Add(_, _, _) => todo!(),
    Instruction::Sub(_, _, _) => todo!(),
    Instruction::Mul(_, _, _) => todo!(),
    Instruction::Div(_, _, _) => todo!(),
    Instruction::ConstInteger(_, _) => todo!(),
    Instruction::ConstFloat(_, _) => todo!(),
    Instruction::CallFunc(_) => todo!(),
    Instruction::Label(_) => todo!(),
    Instruction::Goto(_) => todo!(),
    Instruction::BranchEq(_) => todo!(),
    Instruction::BranchNotEq(_) => todo!(),
    Instruction::BranchLessThan(_) => todo!(),
    Instruction::BranchLessThanEq(_) => todo!(),
    Instruction::BranchGreaterThan(_) => todo!(),
    Instruction::BranchGreaterThanEq(_) => todo!(),
    Instruction::Return(_) => todo!(),
  }
}

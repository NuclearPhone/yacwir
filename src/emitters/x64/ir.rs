#[derive(Clone, Copy)]
pub enum Register {
  RAX,
  RBX,
  RCX,
  RDX,
}

// a location that a value can reside in
#[derive(Clone, Copy)]
pub enum ValLoc {
  Register(Register),
  Memory(usize),

  // the FPU stack
  Float,
}

#[derive(Clone, Copy)]
pub enum Instruction {
  // push a value onto the stack
  Push(ValLoc),

  // pop the top stack value into a location
  Pop(ValLoc),

  // a op b => c
  Add(ValLoc, ValLoc, ValLoc),
  Sub(ValLoc, ValLoc, ValLoc),
  Mul(ValLoc, ValLoc, ValLoc),
  Div(ValLoc, ValLoc, ValLoc),

  // load an integer into a value location
  // TODO: figure out how to support values larger than the word size
  ConstInteger(usize, ValLoc),

  // push a float to the top of the FPU
  ConstFloat(usize, ValLoc),

  // funcidx
  CallFunc(usize),

  // label of the format LL_[int]
  Label(usize),
  Goto(usize),

  BranchEq(usize),
  BranchNotEq(usize),
  BranchLessThan(usize),
  BranchLessThanEq(usize),
  BranchGreaterThan(usize),
  BranchGreaterThanEq(usize),

  // return a value,
  // the final result value is in %rax
  Return(ValLoc),
}

pub struct Function {
  pub name: String,

  // size in bytes of how much space to allocate on the stack
  pub stack_space: usize,

  pub instrs: Vec<Instruction>,
}

pub struct Unit {
  pub funcs: Vec<Function>,
}

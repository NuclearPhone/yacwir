use crate::{ir::Type, token::Span};

type InstrIdx = usize;

pub enum InstructionValue {
  DefineLocals()
  Add(InstrIdx, InstrIdx),
}

pub struct Instruction {}

pub struct Function {
  name: Span,
  parameters: Vec<Type>,
}

/*

===
WLR
===

defn main():
  let x = 2 + 2

===
 IR unoptimized
===

defn main() -> Moot:
  %0 = constint 2 : ComptimeInt
  %1 = constint 2 : ComptimeInt
  %2 = add(2, 2)  : ComptimeInt

===
MIR unoptimized
===

defn main() -> I32:
  LOCALVAR x: uint_size_t = 0;

  Assign(x, 2 + 2)

===
 c
===

int main() {
  uint_t x;
  x = 2 + 2;
}

*/

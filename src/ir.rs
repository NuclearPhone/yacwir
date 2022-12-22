use std::fmt::Display;

use crate::{
  context::CompilerContext,
  token::{Span, TokIdx},
};

/*

defn main():
  let x = 1
  let y = 2
  let z = x * y
  print(z)

main:
  %0 [Integer] = ConstInteger(1)
  %1 [Integer] = ConstInteger(2)
  %2 [Integer] = Mul(%1, %2)
  %3 [Void]    = Call(std::Print, %2)

*/

/*
instruction
Phi(x, y, z, ...)

Tells the IR/Optimizer/Emitter/etc. that
an instruction value can be equivalent a union of multiple values,
to allow conditional flow.

e.g.
int y;

if(1 > 2)
  y = 3;
else
  y = 5;

goes into:

%0 ConstBool(False)
%1 CondJmp(%0, %3)
%2 Jump(%6)

%3 ConstInt(3)
%4 Jump(%6)

%5 ConstInt(5)

%6 Phi(%3, %5)



*/

/*

// BINARY OPERATIONS
binary operations, such as Add, Sub, Mul, Div,
  only apply to integer and floating values within the IR
user defined operator overloads or compiler intrinsic operators
  get transformed into function calls during the Ast2Ir emission process

*/

/*

multiple level of IRs?

- untyped high level representation (HIR)
  generated from the raw AST
  contains dataflow, but as a flat map
  types are not propogated yet

  fun main():
    %0 = ConstInt(0)
    %1 = ConstInt(1)
    %2 = if ( LessThan(%0, %1) ):
            ConstInt(2)
         else:
            ConstInt(3)
    %3 = return %2

- typed high level representation (THIR)
  generated from UTHIR
  contains dataflow, but as a flat map
  where most of the optimizations take place

*/

pub type InstrIdx = usize;
pub type BlockIdx = usize;

#[derive(Debug, Clone)]
pub enum InstructionValue {
  // attempts to perform typecasting to a specified type
  Cast(InstrIdx, Type),

  Assign(InstrIdx),

  // a list of instruction indexes
  // which values can be "moved" into this one
  // used for conditionals, see top of doc

  // left: a block to pull from
  // right: an instruction idx pointing to another Phi/PhiTerminal instructinon
  Phi(BlockIdx, InstrIdx),

  // left: a block to pull from
  // right: a block to pull from
  PhiTerminal(BlockIdx, BlockIdx),

  // constants
  ConstFloat(f64),
  ConstInteger(i64),

  // index into local temps
  Add(InstrIdx, InstrIdx),
  Subtract(InstrIdx, InstrIdx),
  Multiply(InstrIdx, InstrIdx),
  Divide(InstrIdx, InstrIdx),

  Return(InstrIdx),

  Call {
    // index into the token list to an identifier
    name: TokIdx,

    // list of locals to pass as parameters
    params: Vec<InstrIdx>,
  },
}

#[derive(Clone, Debug)]
pub struct Instruction {
  pub val: InstructionValue,

  // index into the instanced token list
  // relative token
  // e.g. Add(%1, %2)  ->  '+'
  pub tok: TokIdx,

  // the associated type with the expression
  // e.g. Add(ConstInt(1), ConstInt(2)) has a type of Integer
  pub ty: Type,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
  // an undecided type, not allowed during codegen
  Undecided,

  // an invalid type, used to communicate that
  // typechecking for an instruction had failed
  Invalid,

  // a 64-bit floating point number
  Floating,

  // a 64-bit signed integer
  Integer,

  // equivalent to a void value
  Moot,
}

// a block is a list of IrInstructions where control flow
// enters at the top (idx. 0) and leaves from different
// specified exit points (IrReturn instructions)
#[derive(Debug)]
pub struct IrBlock(pub Vec<Instruction>);

#[derive(Debug)]
pub struct IrFunction {
  // index into the token array
  pub name: Span,

  // index 0 is the "main" block,
  // where execution begins within this function
  pub blocks: Vec<IrBlock>,
  // TODO: parameters
}

#[derive(Debug)]
pub struct IrUnit {
  pub funcs: Vec<IrFunction>,
}

/// "flattens" a block, by un-fragmenting all of the SSA
/// id values,
/// this is a very expensive function
/// e.g.
///     %1 ConstInt(1)
///     %3 ConstInt(2)
///     %5 Add(1, 3)
/// converts to
///     %0 ConstInt(0)
///     %1 ConstInt(2)
///     %2 Add(0, 1)

pub fn flatten(_unit: IrBlock) -> IrBlock {
  unimplemented!()
}

/*

// optimizing constants

scan through the entire IR maniferst for a function,
ignoring all constants,
when an operation other than a constant is found,
  check its operands
if the operands are constants and can be folded/optimized,
optimize them, push the operands, then push the operator

*/

// beauty print functions

impl Display for InstructionValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str: String = match self {
      Self::ConstInteger(i) => format!("ConstInteger({})", i),
      Self::ConstFloat(f) => format!("ConstFloat({})", f),

      Self::Add(left, right) => format!("Add(%{}, %{})", left, right),
      Self::Subtract(left, right) => format!("Subtract(%{}, %{})", left, right),
      Self::Multiply(left, right) => format!("Multiply(%{}, %{})", left, right),
      Self::Divide(left, right) => format!("Divide(%{}, %{})", left, right),

      Self::Return(ret) => format!("Return(%{})", ret),

      _ => unimplemented!(),
    };

    f.write_str(str.as_str())
  }
}

impl Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // assume that no instruction display will ever get past 40 cols
    // could probably do some length finageling
    let str = format!("{}\x1b[40G as {}", self.val, self.ty);
    f.write_str(str.as_str())
  }
}

impl Display for IrBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.0.len() == 0 {
      return f.write_str("");
    }

    let mut str = String::new();

    for instr in self.0.iter() {
      str.push_str(format!("{}\n", instr).as_str());
    }

    // calculate the maximum length of the idx as a str
    let idx_disp_max_len = {
      let log = self.0.len().ilog10() as usize + 1;

      if log % 2 == 0 {
        log + 1
      } else {
        log
      }
    };

    let out = str
      .lines()
      .fold(
        (String::new(), 0),
        |(mut acc, mut idx): (String, u32), next| {
          acc.push_str(format!("=={:^width$}==\t", idx, width = idx_disp_max_len).as_str());
          acc.push_str(next);
          acc.push('\n');
          idx += 1;
          (acc, idx)
        },
      )
      .0;

    f.write_str(out.as_str())
  }
}

pub struct IrFuncDisplay<'a>(pub &'a CompilerContext, pub &'a IrFunction);
impl<'a> From<(&'a CompilerContext, &'a IrFunction)> for IrFuncDisplay<'a> {
  fn from(input: (&'a CompilerContext, &'a IrFunction)) -> Self {
    Self(input.0, input.1)
  }
}

impl<'a> Display for IrFuncDisplay<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let instrs = format!("{}", self.1.instrs);

    let str = format!(
      "Function <{}>:\n{}",
      self.0.get_str_from_span(self.1.name),
      instrs,
    );

    f.write_str(str.as_str())
  }
}

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Type::Integer => "Integer",
      Type::Floating => "Floating",
      Type::Moot => "Moot",
      Type::Invalid => "Invalid",
      Type::Undecided => "Undecided",
    })
  }
}

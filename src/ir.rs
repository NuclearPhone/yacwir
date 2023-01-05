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

pub type BlockIdx = u32;
pub type FuncIdx = u32;
pub type InstrIdx = u32;

#[derive(Debug, Clone)]
pub struct IntraBlockIdx {
  block: BlockIdx,
  instr: InstrIdx,
}

impl From<(u32, u32)> for IntraBlockIdx {
  fn from(value: (u32, u32)) -> Self {
    Self {
      block: value.0,
      instr: value.1,
    }
  }
}

#[derive(Debug, Clone)]
pub enum InstructionValue {
  // attempts to perform typecasting to a specified type
  Cast(InstrIdx, Type),

  // a list of instruction indexes
  // which values can be "moved" into this one
  // used for conditionals, see top of doc
  // phi functions _MUST_ have at least 2 sources to be implemented,
  // thus this uses S expressions

  // left: a block to pull from
  // right: an instruction idx pointing to another Phi/PhiTerminal instructinon
  Phi(IntraBlockIdx, InstrIdx),

  // left: a block to pull from
  // right: a block to pull from
  PhiTerminal(IntraBlockIdx, IntraBlockIdx),

  // constants
  ConstFloat(f64),
  ConstInteger(i64),

  // index into local temps
  Add(InstrIdx, InstrIdx),
  Subtract(InstrIdx, InstrIdx),
  Multiply(InstrIdx, InstrIdx),
  Divide(InstrIdx, InstrIdx),

  // checks if two values are equal
  // only works on primitive types
  Equal(InstrIdx, InstrIdx),

  // checks a statement, checks if it is true,
  // if it is true, then go to block a,
  // if it is false, then go to block b
  Branch {
    bool_statement: bool,
    block_a: BlockIdx,
    block_b: BlockIdx,
  },

  // returns from a function from within a block
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

#[derive(Debug, Clone)]
pub struct IrFunction {
  pub name: Span,

  // index into the block-span-array (see IrUnit)
  // for the entry block of this function
  pub block: BlockIdx,

  pub ret_type: Type,
}

#[derive(Debug, Default)]
pub struct IrUnit {
  pub instructions: Vec<Instruction>,
  pub blocks: Vec<Span>,
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

impl Display for IntraBlockIdx {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let out = format!("({}, {})", self.block, self.instr);
    f.write_str(&out)
  }
}

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

impl IrUnit {
  pub fn display(&self, ctx: &CompilerContext) -> String {
    let funcs = (0..self.funcs.len())
      .into_iter()
      .fold(String::new(), |mut a, f| {
        a += &self.display_function(ctx, f as u32);
        a
      });

    format!("=== IRUNIT BEGIN ===\n{}=== IRUNIT END===\n", funcs)
  }

  pub fn display_function(&self, ctx: &CompilerContext, function: FuncIdx) -> String {
    let function = &self.funcs[function as usize];
    format!(
      "defn {}\n{}\n",
      ctx.get_str_from_span(function.name),
      self.display_block(function.block)
    )
  }

  // also displays any blocks that it dominates
  pub fn display_block(&self, idx: BlockIdx) -> String {
    let block = self.blocks[idx as usize];

    let instrs = &self.instructions[block.start..block.end];

    let max_num_len = (instrs.len().ilog(10) as usize).max(4);

    let instr_data = instrs
      .iter()
      .fold(
        (block.start, format!("BLOCK #{}\n", idx)),
        |(mut x, mut a), i| {
          a += format!("%{:<max_num_len$} {i}\n", x).as_str();
          x += 1;
          (x, a)
        },
      )
      .1;

    if let InstructionValue::Branch {
      block_a, block_b, ..
    } = instrs.last().unwrap().val
    {
      format!(
        "{}\n{}\n{}\n",
        instr_data,
        self.display_block(block_a),
        self.display_block(block_b)
      )
    } else {
      instr_data
    }
  }
}

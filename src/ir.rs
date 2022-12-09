use std::fmt::Display;

use crate::{
  context::CompilerContext,
  diagnostic::{Diagnostic, DiagnosticLevel},
  emitter::Emitter,
  node::{Binary, FunctionDef, Node, NodeIdx},
  parser::Ast,
  token::TokIdx,
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

pub type InstrIdx = usize;

#[derive(Debug, Clone)]
pub enum InstructionValue {
  // attempts to perform typecasting to a specified type
  Cast(InstrIdx, Type),

  Assign(InstrIdx),

  // a list of instruction indexes
  // which values can be "moved" into this one
  // used for conditionals, see top of doc
  Phi(Vec<InstrIdx>),

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
}

// a block is a list of IrInstructions where control flow
// enters at the top (idx. 0) and leaves from different
// specified exit points (IrReturn instructions)
#[derive(Debug)]
pub struct IrBlock(pub Vec<Instruction>);

#[derive(Debug)]
pub struct IrFunction {
  // index into the token array
  pub name: TokIdx,

  pub instrs: IrBlock,
}

#[derive(Debug)]
pub struct IrUnit {
  pub funcs: Vec<IrFunction>,
}

pub struct IrEmitter<'a, 'b> {
  ctx: &'a CompilerContext,
  ast: &'b Ast<'a>,
}

impl<'a, 'b> IrEmitter<'a, 'b> {
  fn emit_binary(
    &mut self,
    binary: &Binary,
    buffer: &mut Vec<Instruction>,
  ) -> Result<(InstrIdx, InstrIdx), String> {
    let l = self.emit_node(binary.left, buffer)?;
    let r = self.emit_node(binary.right, buffer)?;
    Ok((l, r))
  }

  fn emit_node(
    &mut self,
    nidx: NodeIdx,
    buffer: &mut Vec<Instruction>,
  ) -> Result<InstrIdx, String> {
    let node = &self.ast.nodes[nidx];

    let (instr_val, instr_ty): (InstructionValue, Type) = match node {
      Node::Floating { val, .. } => (InstructionValue::ConstFloat(*val), Type::Floating),

      Node::Add(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Add(l, r), Type::Undecided)
      }

      Node::Subtract(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Subtract(l, r), Type::Undecided)
      }

      Node::Multiply(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Multiply(l, r), Type::Undecided)
      }

      Node::Divide(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Divide(l, r), Type::Undecided)
      }

      Node::Block(block) => {
        // block have no Instruction representation,
        // and they should semantically never be transformed as a value
        // thus just return a 0 and hope for the best
        for nidx in block {
          self.emit_node(*nidx, buffer)?;
        }

        return Ok(0);
      }

      Node::Return(ret) => {
        let expr = self.emit_node(*ret, buffer)?;
        (InstructionValue::Return(expr), Type::Undecided)
      }

      _ => return Err(format!("unknown node in ast->ir emitter {:?}", node)),
    };

    buffer.push(Instruction {
      val: instr_val,

      ty: instr_ty,
      // TODO: implement this
      tok: 0,
    });

    Ok(buffer.len() - 1)
  }

  fn emit_function(&mut self, nidx: NodeIdx) -> Result<IrFunction, String> {
    let Node::FunctionDef(node) = self.ast.nodes.get(nidx).unwrap() else { panic!(); };

    let mut buf = vec![];
    self.emit_node(node.exec, &mut buf)?;

    Ok(IrFunction {
      name: node.name,
      instrs: IrBlock(buf),
    })
  }

  fn emit_unit(&mut self) -> Result<IrUnit, String> {
    // node-idx 0 is guaranteed to be the function 'main', so start there
    let main = self.emit_function(0)?;

    Ok(IrUnit { funcs: vec![main] })
  }
}

impl<'a, 'b> Emitter<'a, 'b> for IrEmitter<'a, 'b> {
  type Input = Ast<'a>;
  type Output = Result<IrUnit, String>;

  fn emit(ctx: &'a CompilerContext, ast: &'b Ast<'a>) -> Self::Output {
    Self { ctx, ast }.emit_unit()
  }
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

pub fn flatten(unit: IrBlock) -> IrBlock {
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
    let str = format!("{}\t as {}", self.val, self.ty);
    f.write_str(str.as_str())
  }
}

impl Display for IrBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut str = String::new();

    for instr in self.0.iter() {
      str.push_str(format!("{}\n", instr).as_str());
    }

    f.write_str(str.as_str())
  }
}

impl Display for IrFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let instrs = format!("{}", self.instrs);
    let instrs_format = instrs
      .lines()
      .fold((String::new(), 0), |mut a: (String, u32), i| {
        a.0 += &format!("%{} =\t", a.1);
        a.0 += i;
        a.0 += "\n";
        a.1 += 1;
        a
      });

    let str = format!("Function <{}>:\n{}", self.name, instrs_format.0);

    f.write_str(str.as_str())
  }
}

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Type::Integer => "Integer",
      Type::Floating => "Floating",
      Type::Invalid => "Invalid",
      Type::Undecided => "Undecided",
    })
  }
}

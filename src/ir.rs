use std::fmt::Display;

use crate::{
  context::CompilerContext,
  diagnostic::{Diagnostic, DiagnosticLevel},
  emitter::Emitter,
  node::{Binary, FunctionDef, Node, NodeIdx},
  parser::Ast,
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
pub type InstrIdx = usize;

#[derive(Debug, Clone)]
pub enum InstructionValue {
  Assign(InstrIdx),

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
    // TODO: prolly convert this to a &'a str

    // name of function to call
    name: String,

    // list of locals to pass as parameters
    params: Vec<InstrIdx>,
  },
}

#[derive(Clone, Debug)]
pub struct Instruction {
  pub id: usize,
  pub val: InstructionValue,
}

// a block is a list of IrInstructions where control flow
// enters at the top (idx. 0) and leaves from different
// specified exit points (IrReturn instructions)
#[derive(Debug)]
pub struct IrBlock(pub Vec<Instruction>);

#[derive(Debug)]
pub struct IrFunction {
  pub name: String,
  pub instrs: IrBlock,
}

#[derive(Debug)]
pub struct IrUnit {
  pub funcs: Vec<IrFunction>,
}

pub struct IrEmitter<'a> {
  ctx: &'a CompilerContext,
  ast: &'a Ast<'a>,
}

impl<'a> IrEmitter<'a> {
  fn emit_binary(
    &mut self,
    binary: &Binary,
    buffer: &mut Vec<Instruction>,
  ) -> Result<(InstrIdx, InstrIdx), ()> {
    let l = self.emit_node(binary.left, buffer)?;
    let r = self.emit_node(binary.right, buffer)?;
    Ok((l, r))
  }

  fn emit_node(&mut self, nidx: NodeIdx, buffer: &mut Vec<Instruction>) -> Result<InstrIdx, ()> {
    let node = &self.ast.nodes[nidx];

    let instr_val: InstructionValue = match node {
      Node::Floating { val, .. } => InstructionValue::ConstFloat(*val),

      Node::Add(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        InstructionValue::Add(l, r)
      }

      Node::Subtract(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        InstructionValue::Subtract(l, r)
      }

      Node::Multiply(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        InstructionValue::Multiply(l, r)
      }

      Node::Divide(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        InstructionValue::Divide(l, r)
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

      _ => return Err(()),
    };

    buffer.push(Instruction {
      id: buffer.len(),
      val: instr_val,
    });

    Ok(buffer.len() - 1)
  }

  fn emit_function(&mut self, nidx: NodeIdx) -> Result<IrFunction, ()> {
    let Node::FunctionDef(node) = self.ast.nodes.get(nidx).unwrap() else { panic!(); };

    let mut buf = vec![];
    self.emit_node(node.exec, &mut buf)?;

    Ok(IrFunction {
      name: node.name.clone(),
      instrs: IrBlock(buf),
    })
  }

  fn emit_unit(&mut self) -> Result<IrUnit, ()> {
    // node-idx 0 is guaranteed to be the function 'main', so start there
    let main = self.emit_function(0).unwrap();

    Ok(IrUnit { funcs: vec![main] })
  }
}

impl<'a> Emitter<'a> for IrEmitter<'a> {
  type Input = Ast<'a>;
  type Output = Result<IrUnit, ()>;

  fn new(ctx: &'a crate::context::CompilerContext, ast: &'a Self::Input) -> Self {
    Self { ctx, ast }
  }

  fn emit(mut self) -> Self::Output {
    self.emit_unit()
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

      _ => unimplemented!(),
    };

    f.write_str(str.as_str())
  }
}

impl Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = format!("%{}\t{}", self.id, self.val);
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

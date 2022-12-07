use crate::{
  context::CompilerContext,
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

#[derive(Debug)]
pub enum Instruction {
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
    // name of function to call
    name: String,

    // list of locals to pass as parameters
    params: Vec<InstrIdx>,
  },
}

#[derive(Debug)]
pub struct IrFunction {
  pub name: String,
  pub instrs: Vec<Instruction>,
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

    match node {
      Node::Floating { val, .. } => buffer.push(Instruction::ConstFloat(*val)),

      Node::Add(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        buffer.push(Instruction::Add(l, r));
      }

      Node::Subtract(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        buffer.push(Instruction::Subtract(l, r));
      }

      Node::Multiply(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        buffer.push(Instruction::Multiply(l, r));
      }

      Node::Divide(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        buffer.push(Instruction::Divide(l, r));
      }

      Node::Block(block) => {
        for nidx in block {
          self.emit_node(*nidx, buffer)?;
        }
      }

      _ => return Err(()),
    }

    Ok(buffer.len() - 1)
  }

  fn emit_function(&mut self, nidx: NodeIdx) -> Result<IrFunction, ()> {
    let Node::FunctionDef(node) = self.ast.nodes.get(nidx).unwrap() else { panic!(); };

    let mut buf = vec![];
    self.emit_node(node.exec, &mut buf)?;

    Ok(IrFunction {
      name: node.name.clone(),
      instrs: buf,
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

/*

// optimizing constants

scan through the entire IR maniferst for a function,
ignoring all constants,
when an operation other than a constant is found,
  check its operands
if the operands are constants and can be folded/optimized,
optimize them, push the operands, then push the operator

*/

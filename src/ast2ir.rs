// converts the Ast representation of source code
// to a variant of SSA form

use crate::{
  ir::{InstrIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit, Type},
  node::{Binary, Node, NodeData, NodeIdx},
  parser::Ast,
};

pub struct IrEmitter<'a> {
  // ast already holds onto a context
  ast: &'a Ast,
}

impl<'a> IrEmitter<'a> {
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

    let (instr_val, instr_ty): (InstructionValue, Type) = match &node.data {
      NodeData::Floating(val) => (InstructionValue::ConstFloat(*val), Type::Floating),

      NodeData::Add(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Add(l, r), Type::Undecided)
      }

      NodeData::Subtract(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Subtract(l, r), Type::Undecided)
      }

      NodeData::Multiply(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Multiply(l, r), Type::Undecided)
      }

      NodeData::Divide(bin) => {
        let (l, r) = self.emit_binary(bin, buffer)?;
        (InstructionValue::Divide(l, r), Type::Undecided)
      }

      NodeData::Block(block) => {
        // block have no Instruction representation,
        // and they should semantically never be transformed as a value
        // thus just return a 0 and hope for the best
        for nidx in block {
          self.emit_node(*nidx, buffer)?;
        }

        return Ok(0);
      }

      NodeData::Return(ret) => {
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
    let Some(Node{data: NodeData::FunctionDef(node), ..}) = self.ast.nodes.get(nidx) else { panic!(); };

    let mut buf = vec![];
    self.emit_node(node.exec, &mut buf)?;

    Ok(IrFunction {
      name: node.name,
      instrs: IrBlock(buf),
    })
  }

  fn emit_unit(&mut self) -> Result<IrUnit, String> {
    // the node in idx 0 can only be a functiondef if and only if defn main is defined
    let Node {data: NodeData::FunctionDef(_), ..} = self.ast.nodes[0] else {
      return Err("Main function is not defined".to_string());
    };

    // node-idx 0 is guaranteed to be the function 'main', so start there
    let main = self.emit_function(0)?;

    Ok(IrUnit { funcs: vec![main] })
  }

  pub fn emit(ast: &'a Ast) -> Result<IrUnit, String> {
    Self { ast }.emit_unit()
  }
}

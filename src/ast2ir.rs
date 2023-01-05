// converts the Ast representation of source code
// to a variant of SSA form

use std::sync::Arc;

use crate::{
  context::CompilerContext,
  ir::{BlockIdx, InstrIdx, Instruction, InstructionValue, IrBlock, IrFunction, IrUnit, Type},
  node::{Binary, FunctionDef, Node, NodeData, NodeIdx},
  parser::Ast,
  sema::SemaContext,
  token::Span,
};

pub fn emit(ctx: &CompilerContext, sema: &mut SemaContext, ast: &Ast) -> IrUnit {
  let mut instructions = vec![];
  let mut blocks = vec![];
  let mut funcs = vec![];

  // the first ast function is guaranteed to be the "main" function
  // thus the first function here is also guaranteed to be the main function
  for funcidx in ast.funcs.iter() {
    let NodeData::FunctionDef(func) = ast.nodes[*funcidx].data else { panic!()};

    funcs.push(IrFunction {
      name: func.name,
      block: IrBlockEmitter::emit(ast, func.exec, &mut instructions, &mut blocks).unwrap(),
      ret_type: emit_type(sema, func.return_type),
    });
  }

  IrUnit {
    instructions,
    blocks,
    funcs,
  }
}

fn emit_type(sema: &mut SemaContext, ty: crate::node::Type) -> crate::ir::Type {
  match ty {
    crate::node::Type::Undecided => crate::ir::Type::Undecided,
    crate::node::Type::Integer => crate::ir::Type::Integer,
    crate::node::Type::Floating => crate::ir::Type::Floating,
    crate::node::Type::Moot => crate::ir::Type::Moot,
  }
}

// TODO: implement conditional/return instruction/blocks
// - implement recursion for IrBlockEmitter
// - at an end of branch instruction, create a new instance of IrBlockEmitter
//     for each block, then call and return

pub struct IrBlockEmitter<'a> {
  ast: &'a Ast,
  ast_block: &'a Vec<NodeIdx>,

  instructions: &'a mut Vec<Instruction>,
  blocks: &'a mut Vec<Span>,
}

impl<'a> IrBlockEmitter<'a> {
  fn emit_binary(&mut self, binary: Binary) -> Result<(InstrIdx, InstrIdx), String> {
    let l = self.emit_node(binary.left)?;
    let r = self.emit_node(binary.right)?;
    Ok((l, r))
  }

  fn emit_node(&mut self, nidx: NodeIdx) -> Result<InstrIdx, String> {
    let node = &self.ast.nodes.get(nidx).unwrap();

    let (instr_val, instr_ty): (InstructionValue, Type) = match &node.data {
      NodeData::Floating(val) => (InstructionValue::ConstFloat(*val), Type::Floating),

      NodeData::Add(bin) => {
        let (l, r) = self.emit_binary(*bin)?;
        (InstructionValue::Add(l, r), Type::Undecided)
      }

      NodeData::Subtract(bin) => {
        let (l, r) = self.emit_binary(*bin)?;
        (InstructionValue::Subtract(l, r), Type::Undecided)
      }

      NodeData::Multiply(bin) => {
        let (l, r) = self.emit_binary(*bin)?;
        (InstructionValue::Multiply(l, r), Type::Undecided)
      }

      NodeData::Divide(bin) => {
        let (l, r) = self.emit_binary(*bin)?;
        (InstructionValue::Divide(l, r), Type::Undecided)
      }

      NodeData::Block(block) => {
        // block have no Instruction representation,
        // and they should semantically never be transformed as a value
        // thus just return a 0 and hope for the best
        for nidx in block {
          self.emit_node(*nidx)?;
        }

        return Ok(0 as u32);
      }

      NodeData::Return(ret) => {
        let expr = self.emit_node(*ret)?;
        (InstructionValue::Return(expr), Type::Undecided)
      }

      _ => return Err(format!("unknown node in ast->ir emitter {:?}", node)),
    };

    self.instructions.push(Instruction {
      val: instr_val,

      ty: instr_ty,
      // TODO: implement this
      tok: node.tok,
    });

    Ok((self.instructions.len() - 1) as u32)
  }

  fn inner_emit(mut self) -> Result<BlockIdx, String> {
    let blockidx = self.blocks.len();

    let start = self.instructions.len();
    for nidx in self.ast_block {
      _ = self.emit_node(*nidx)?;
    }
    let end = self.instructions.len();

    self.blocks.push(Span { start, end });

    Ok(blockidx as u32)
  }

  fn emit(
    ast: &'a Ast,
    ast_idx: NodeIdx,
    instructions: &'a mut Vec<Instruction>,
    blocks: &'a mut Vec<Span>,
  ) -> Result<BlockIdx, String> {
    let NodeData::Block(ref ast_block) = ast.nodes.get(ast_idx).unwrap().data else { panic!()};

    Self {
      ast,
      ast_block,
      instructions,
      blocks,
    }
    .inner_emit()
  }
}

use crate::{
  ir::Type,
  token::{Span, TokIdx},
};

pub type NodeIdx = usize;

#[derive(Debug)]
pub struct Node {
  // the actual data of this node
  pub data: NodeData,

  // root token associated with this node,
  // used for diagnostics and debugging
  pub tok: TokIdx,
}

#[derive(Debug)]
pub enum NodeData {
  Add(Binary),
  Subtract(Binary),
  Multiply(Binary),
  Divide(Binary),

  Floating(f64),

  Identifier(Span),

  FunctionDef(FunctionDef),
  Block(Vec<NodeIdx>),

  // return a value
  Return(NodeIdx),
}

#[derive(Debug)]
pub struct Binary {
  pub left: NodeIdx,
  pub right: NodeIdx,
}

#[derive(Debug)]
pub struct FunctionDef {
  pub name: Span,

  // index to a block of nodes
  pub exec: NodeIdx,
}

pub type ParameterDeclList<'a> = Vec<(&'a str, Option<Type>)>;

// impl Node {
//   fn display(&self, ctx: &CompilerContext, indentation: usize, buffer: &mut String) {
//     buffer.push_str(" ".repeat(indentation).as_str());

//     let out = match self {
//       Self::Floating{ val: f64} => format!("{}", f),

//       _ => unimplemented!()
//     }
//   }
// }

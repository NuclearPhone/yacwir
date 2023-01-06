use crate::token::{Span, TokIdx};

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
  // a value with no data
  Moot,

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

#[derive(Debug, Clone, Copy)]
pub struct Binary {
  pub left: NodeIdx,
  pub right: NodeIdx,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
  pub name: Span,
  pub params: Vec<(Span, Type)>,
  pub return_type: Type,
  pub exec: NodeIdx,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimType {
  Undecided,

  // usize => bitwidth
  Integer(usize),
  Unsigned(usize),
  Floating(usize),

  Moot,

  UserDef,
}

#[derive(Debug, Clone, Copy)]
pub struct Type {
  pub prim: PrimType,
  pub ptr: usize,
}

impl From<PrimType> for Type {
  fn from(value: PrimType) -> Self {
    Self {
      prim: value,
      ptr: 0,
    }
  }
}

// impl Node {
//   fn display(&self, ctx: &CompilerContext, indentation: usize, buffer: &mut String) {
//     buffer.push_str(" ".repeat(indentation).as_str());

//     let out = match self {
//       Self::Floating{ val: f64} => format!("{}", f),

//       _ => unimplemented!()
//     }
//   }
// }

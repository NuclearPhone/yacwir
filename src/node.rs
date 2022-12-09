use crate::{context::CompilerContext, ir::Type, token::TokIdx};

pub type NodeIdx = usize;

#[derive(Debug)]
pub enum Node {
  Add(Binary),
  Subtract(Binary),
  Multiply(Binary),
  Divide(Binary),

  Floating { val: f64, tokidx: usize },

  // index into the token array
  Identifier(usize),

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
  pub name: TokIdx,

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

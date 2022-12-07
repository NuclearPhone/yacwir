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
}

#[derive(Debug)]
pub struct Binary {
  pub left: NodeIdx,
  pub right: NodeIdx,
}

#[derive(Debug)]
pub struct FunctionDef {
  pub name: String,

  // index to a block of nodes
  pub exec: NodeIdx,
}

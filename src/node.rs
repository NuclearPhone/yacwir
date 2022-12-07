#[derive(Debug)]
pub enum Node {
  Add(Binary),
  Subtract(Binary),
  Multiply(Binary),
  Divide(Binary),

  Floating { val: f64, tokidx: usize },

  // index into the token array
  Identifier(usize),
}

#[derive(Debug)]
pub struct Binary {
  pub left: Box<Node>,
  pub right: Box<Node>,
}

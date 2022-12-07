#[derive(Debug, Clone)]
pub enum TokenType {
  Number,
  Identifier,

  Plus,
  Minus,
  Asterisk,
  Solidus,

  LeftParanthesis,
  RightParanthesis,

  // used for scope
  Indentation,

  // custom token that does not match to any rule in the parser,
  // used for early returns
  EOF,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
  pub ty: TokenType,

  // slice representing the text of the token
  pub slice: &'a str,
}

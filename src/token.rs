use std::fmt::Display;

pub type TokIdx = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Number,
  Identifier,

  Plus,
  Minus,
  Asterisk,
  Solidus,

  LeftParanthesis,
  RightParanthesis,

  Colon,

  // used for scope
  Indentation,

  // keywords
  Return,
  Defn,

  Comment,

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

impl Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      TokenType::Number => "number",
      TokenType::Identifier => "identifier",

      TokenType::Plus => "plus",
      TokenType::Minus => "minus",
      TokenType::Asterisk => "asterisk",
      TokenType::Solidus => "solidus",

      TokenType::LeftParanthesis => "left paranthesis",
      TokenType::RightParanthesis => "right paranthesis",

      TokenType::Colon => "colon",

      TokenType::Indentation => "indentation",

      TokenType::Return => "return",
      TokenType::Defn => "defn",

      TokenType::Comment => "comment",

      TokenType::EOF => "EOF",
      _ => unimplemented!(),
    })
  }
}

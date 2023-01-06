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

  ThinArrow,

  Colon,

  Comma,

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

// indexes into the compiled file,
// avoids lifetime hell
// TODO: mark this with phantomdata to ensure memory safety
#[derive(Debug, Clone, Copy)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Span {
  pub fn len(&self) -> usize {
    self.end - self.start
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  pub ty: TokenType,

  // a span representing the text of this token
  pub span: Span,
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
      TokenType::Comma => "comma",

      TokenType::ThinArrow => "->",

      TokenType::Indentation => "indentation",

      TokenType::Return => "return",
      TokenType::Defn => "defn",

      TokenType::Comment => "comment",

      TokenType::EOF => "EOF",
    })
  }
}

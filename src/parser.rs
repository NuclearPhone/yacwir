use crate::{
  context::CompilerContext,
  lexer::Lexer,
  node::{Binary, Node},
  token::{Token, TokenType},
};

pub struct Parser<'a> {
  ctx: &'a CompilerContext,
  toks: Vec<Token<'a>>,
  tokidx: usize,
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a CompilerContext) -> Result<Self, String> {
    Ok(Self {
      ctx,
      toks: Lexer::new(ctx).lex()?,
      tokidx: 0,
    })
  }

  fn current_tok(&self) -> Option<Token<'a>> {
    self.toks.get(self.tokidx.clone()).map(|x| x.clone())
  }

  fn parse_factor(&mut self) -> Result<Node, String> {
    match self
      .current_tok()
      .ok_or("expected token while parsing factor")?
    {
      Token {
        ty: TokenType::Number,
        slice,
      } => {
        self.tokidx += 1;

        Ok(Node::Floating {
          val: slice
            .parse::<f64>()
            .ok()
            .ok_or("error while trying to parse a number".to_owned())?,

          tokidx: self.tokidx - 1,
        })
      }

      _ => Err("unknown symbol in parse_factor".into()),
    }
  }

  fn parse_term(&mut self) -> Result<Node, String> {
    let mut left = self.parse_factor()?;

    while let Some(Token {
      ty: TokenType::Asterisk | TokenType::Solidus,
      ..
    }) = self.current_tok()
    {
      let ty = self.current_tok().unwrap().ty;
      self.tokidx += 1;

      let right = self.parse_factor()?;

      match ty {
        TokenType::Asterisk => {
          left = Node::Multiply(Binary {
            left: Box::new(left),
            right: Box::new(right),
          })
        }

        TokenType::Solidus => {
          left = Node::Divide(Binary {
            left: Box::new(left),
            right: Box::new(right),
          })
        }

        _ => unreachable!(),
      }
    }

    Ok(left)
  }

  pub fn parse(mut self) -> Result<Node, String> {
    self.parse_term()
  }
}

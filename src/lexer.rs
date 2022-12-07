use crate::{
  context::CompilerContext,
  token::{Token, TokenType},
};

pub struct Lexer<'a> {
  input: &'a str,
  idx: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(ctx: &'a CompilerContext) -> Self {
    Self {
      input: ctx.get_input_str(),
      idx: 0,
    }
  }

  fn _current_char(&self) -> Option<char> {
    self.input.chars().nth(self.idx)
  }

  fn _skip_whitespace(&mut self) {
    while let Some(ch) = self._current_char() {
      if !ch.is_whitespace() {
        break;
      }
      self.idx += 1;
    }
  }

  pub fn _lex<'b>(&'b mut self) -> Result<Token<'a>, String> {
    self._skip_whitespace();

    match self
      ._current_char()
      .ok_or("Ran out of characters while in _lex")?
    {
      '+' => {
        self.idx += 1;
        return Ok(Token {
          ty: TokenType::Plus,
          slice: &self.input[self.idx..self.idx],
        });
      }

      '-' => {
        self.idx += 1;
        return Ok(Token {
          ty: TokenType::Minus,
          slice: &self.input[self.idx..self.idx],
        });
      }

      '*' => {
        self.idx += 1;
        return Ok(Token {
          ty: TokenType::Asterisk,
          slice: &self.input[self.idx..self.idx],
        });
      }

      '/' => {
        self.idx += 1;
        return Ok(Token {
          ty: TokenType::Solidus,
          slice: &self.input[self.idx..self.idx],
        });
      }

      x if x.is_digit(10) => {
        let mut len = 0;

        while let Some(ch) = self._current_char() {
          if !ch.is_digit(10) && ch != '.' {
            break;
          }

          len += 1;
          self.idx += 1;
        }

        return Ok(Token {
          ty: TokenType::Number,
          slice: &self.input[self.idx - len..self.idx],
        });
      }

      _ => return Err("fallthrough in lexer function".into()),
    }
  }

  pub fn lex(mut self) -> Result<Vec<Token<'a>>, String> {
    let mut toks = vec![];

    while self.idx < self.input.len() {
      toks.push(self._lex()?);
    }

    toks.push(Token {
      ty: TokenType::EOF,
      slice: &self.input[self.input.len()..self.input.len()],
    });

    Ok(toks)
  }
}

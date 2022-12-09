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
      if ch != ' ' {
        break;
      }

      self.idx += 1;
    }
  }

  // assume the \n has already been lexed
  fn _lex_indent<'b>(&'b mut self) -> Result<Token<'a>, String> {
    let mut ind: usize = 0;

    while let Some(ch) = self._current_char() {
      if ch != ' ' {
        break;
      }
      ind += 1;
      self.idx += 1;
    }

    return Ok(Token {
      ty: TokenType::Indentation,
      slice: &self.input[self.idx - ind..self.idx],
    });
  }

  fn _lex<'b>(&'b mut self) -> Result<Token<'a>, String> {
    self._skip_whitespace();

    match self
      ._current_char()
      .ok_or("Ran out of characters while in _lex")?
    {
      '\n' => {
        self.idx += 1;
        return self._lex_indent();
      }

      '+' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Plus,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      '-' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Minus,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      '*' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Asterisk,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      '/' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Solidus,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      ':' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Colon,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      '(' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::LeftParanthesis,
          slice: &self.input[self.idx - 1..self.idx],
        })
      }

      ')' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::RightParanthesis,
          slice: &self.input[self.idx - 1..self.idx],
        })
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

        Ok(Token {
          ty: TokenType::Number,
          slice: &self.input[self.idx - len..self.idx],
        })
      }

      x if x.is_alphabetic() => {
        let mut len = 0;
        while let Some(ch) = self._current_char() {
          if !ch.is_alphanumeric() && ch != '_' {
            break;
          }
          len += 1;
          self.idx += 1;
        }

        let slice = &self.input[self.idx - len..self.idx];

        Ok(Token {
          slice,
          ty: match slice {
            "return" => TokenType::Return,

            "defn" => TokenType::Defn,

            _ => TokenType::Identifier,
          },
        })
      }

      _ => Err(format!(
        "Fallthrough in lexer function: <{}>",
        self._current_char().unwrap()
      )),
    }
  }

  pub fn lex(mut self) -> Result<Vec<Token<'a>>, String> {
    let mut toks = vec![];

    // lex a single indentation
    toks.push(self._lex_indent()?);

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

use crate::{
  context::CompilerContext,
  token::{Span, Token, TokenType},
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
  fn _lex_indent<'b>(&'b mut self) -> Result<Token, String> {
    if self.input[self.idx..].trim_start().starts_with('#') {
      self._skip_whitespace();
      let mut len: usize = 0;
      while let Some(x) = self._current_char() {
        if x == '\n' {
          break;
        }

        len += 1;
        self.idx += 1;
      }
      Ok(Token {
        ty: TokenType::Comment,
        span: Span {
          start: self.idx - len,
          end: self.idx,
        },
      })
    } else {
      self.idx += 1;
      let mut ind: usize = 0;

      while let Some(ch) = self._current_char() {
        // ignore empty line
        if ch == '\n' {
          ind = 0;
          self.idx += 1;
          continue;
        }

        if ch != ' ' {
          break;
        }

        ind += 1;
        self.idx += 1;
      }

      Ok(Token {
        ty: TokenType::Indentation,
        span: Span {
          start: self.idx - ind,
          end: self.idx,
        },
      })
    }
  }

  fn _lex<'b>(&'b mut self) -> Result<Token, String> {
    self._skip_whitespace();

    match self
      ._current_char()
      .ok_or("Ran out of characters while in _lex")?
    {
      '\n' => {
        self.idx += 1;
        self._lex_indent()
      }

      '+' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Plus,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      '-' => {
        self.idx += 1;

        if self
          ._current_char()
          .ok_or("Ran out of characters while in lex")?
          == '>'
        {
          self.idx += 1;
          Ok(Token {
            ty: TokenType::ThinArrow,
            span: Span {
              start: self.idx - 2,
              end: self.idx,
            },
          })
        } else {
          Ok(Token {
            ty: TokenType::Minus,
            span: Span {
              start: self.idx - 1,
              end: self.idx,
            },
          })
        }
      }

      '*' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Asterisk,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      '/' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Solidus,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      ':' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Colon,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      ',' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::Comma,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      '(' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::LeftParanthesis,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
        })
      }

      ')' => {
        self.idx += 1;
        Ok(Token {
          ty: TokenType::RightParanthesis,
          span: Span {
            start: self.idx - 1,
            end: self.idx,
          },
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
          span: Span {
            start: self.idx - len,
            end: self.idx,
          },
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

        let span = Span {
          start: self.idx - len,
          end: self.idx,
        };

        let slice = &self.input[span.start..span.end];

        Ok(Token {
          ty: match slice {
            "return" => TokenType::Return,
            "defn" => TokenType::Defn,

            _ => TokenType::Identifier,
          },

          span,
        })
      }

      _ => Err(format!(
        "Fallthrough in lexer function: <{}>",
        self._current_char().unwrap()
      )),
    }
  }

  pub fn lex(mut self) -> Result<Vec<Token>, String> {
    let mut toks = vec![];

    // lex a single indentation
    toks.push(self._lex_indent()?);

    while self.idx < self.input.len() {
      toks.push(self._lex()?);
    }

    toks.push(Token {
      ty: TokenType::EOF,
      span: Span {
        start: self.input.len(),
        end: self.input.len(),
      },
    });

    Ok(toks)
  }
}

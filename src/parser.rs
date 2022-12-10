use std::fmt::Display;

use crate::{
  context::CompilerContext,
  lexer::Lexer,
  node::{Binary, FunctionDef, Node, NodeData, NodeIdx, ParameterDeclList},
  token::{TokIdx, Token, TokenType},
};

pub struct Ast<'a> {
  pub toks: Vec<Token<'a>>,
  pub nodes: Vec<Node>,

  // list of indices into self.nodes
  // guaranteed to be FunctionDef nodes
  pub funcs: Vec<NodeIdx>,
}

pub struct Parser<'a> {
  ctx: &'a CompilerContext,
  nodes: Vec<Node>,
  toks: Vec<Token<'a>>,
  funcs: Vec<NodeIdx>,
  tokidx: usize,
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a CompilerContext) -> Result<Self, String> {
    Ok(Self {
      ctx,
      nodes: vec![],
      funcs: vec![],
      toks: Lexer::new(ctx).lex()?,
      tokidx: 0,
    })
  }

  fn expect(&mut self, expected_type: TokenType) -> Result<TokIdx, String> {
    // skip any comments
    while let Some(Token {
      ty: TokenType::Comment,
      ..
    }) = self.current_tok()
    {
      self.tokidx += 1;
    }

    let Some(tok) = self.current_tok() else {
      return Err("Ran out of characters to expect".to_string())
    };

    if tok.ty == expected_type {
      self.tokidx += 1;
      Ok(self.tokidx - 1)
    } else {
      Err(format!(
        "Expected token {}, but found {}",
        expected_type, tok.ty
      ))
    }
  }

  fn next_tok(&mut self) -> Token {
    while let Token {
      ty: TokenType::Comment,
      ..
    } = self.toks[self.tokidx]
    {
      self.tokidx += 1;
    }
    let out = self.tokidx;
    self.tokidx += 1;
    self.toks[out].clone()
  }

  fn push_node(&mut self, node: Node) -> usize {
    self.nodes.push(node);
    self.nodes.len() - 1
  }

  fn current_tok(&self) -> Option<Token<'a>> {
    self.toks.get(self.tokidx.clone()).map(|x| x.clone())
  }

  fn parse_factor(&mut self) -> Result<NodeIdx, String> {
    match self
      .current_tok()
      .ok_or("expected token while parsing factor")?
    {
      Token {
        ty: TokenType::Number,
        slice,
      } => {
        self.tokidx += 1;

        Ok(
          self.push_node(Node {
            data: NodeData::Floating(
              slice
                .parse::<f64>()
                .ok()
                .ok_or("error while trying to parse a number".to_owned())?,
            ),
            tok: self.tokidx,
          }),
        )
      }

      _ => Err(format!(
        "unknown symbol in parse_factor <{:?}>",
        self.current_tok()
      )),
    }
  }

  fn parse_term(&mut self) -> Result<NodeIdx, String> {
    let mut left = self.parse_factor()?;

    while let Some(Token {
      ty: TokenType::Asterisk | TokenType::Solidus,
      ..
    }) = self.current_tok()
    {
      let tokidx = self.tokidx;
      let ty = self.current_tok().unwrap().ty;
      self.tokidx += 1;

      let right = self.parse_factor()?;

      left = self.push_node(Node {
        data: match ty {
          TokenType::Asterisk => NodeData::Multiply(Binary { left, right }),
          TokenType::Solidus => NodeData::Divide(Binary { left, right }),

          _ => unreachable!(),
        },
        tok: tokidx,
      });
    }

    Ok(left)
  }

  fn parse_expr(&mut self) -> Result<NodeIdx, String> {
    let mut left = self.parse_term()?;

    while let Some(Token {
      ty: TokenType::Plus | TokenType::Minus,
      ..
    }) = self.current_tok()
    {
      let tokidx = self.tokidx;
      let ty = self.next_tok().ty;

      let right = self.parse_term()?;

      left = self.push_node(Node {
        data: match ty {
          TokenType::Plus => NodeData::Add(Binary { left, right }),
          TokenType::Minus => NodeData::Subtract(Binary { left, right }),

          _ => unreachable!(),
        },
        tok: tokidx,
      });
    }

    Ok(left)
  }

  fn parse_return(&mut self) -> Result<NodeIdx, String> {
    let root_tokidx = self.tokidx;

    let Some(Token{ty: TokenType::Return, ..}) = self.current_tok() else {
      return Err("Expected a return token while parsing return.".to_owned()); 
    };

    self.tokidx += 1;

    let ret_val = self.parse_expr()?;
    Ok(self.push_node(Node {
      data: NodeData::Return(ret_val),
      tok: root_tokidx,
    }))
  }

  fn parse_expr_statement(&mut self) -> Result<NodeIdx, String> {
    if let Some(Token {
      ty: TokenType::Return,
      ..
    }) = self.current_tok()
    {
      self.parse_return()
    } else {
      self.parse_expr()
    }
  }

  fn parse_block(&mut self) -> Result<NodeIdx, String> {
    let root_tokidx = self.tokidx;

    // get indentation
    let base_indentation_idx = self.expect(TokenType::Indentation)?;
    let base_indentation = &self.toks[base_indentation_idx];
    let inden_len = base_indentation.slice.len();

    let mut toks = vec![];

    'l: loop {
      toks.push(self.parse_expr_statement()?);

      let new_ind: usize = match self.current_tok() {
        Some(Token {
          ty: TokenType::Indentation,
          slice,
        }) => slice.len(),

        // break out of parsing loop on EOF
        Some(Token {
          ty: TokenType::EOF, ..
        }) => break 'l,

        _ => return Err("expected an indentation while parsing block".into()),
      };

      if new_ind < inden_len {
        break;
      } else if new_ind > inden_len {
        return Err("somehow tried to parse a larger indentation while parsing a block".into());
      }

      self.tokidx += 1;
    }

    Ok(self.push_node(Node {
      data: NodeData::Block(toks),
      tok: root_tokidx,
    }))
  }

  fn parse_parameter_declaration(&mut self) -> Result<ParameterDeclList, String> {
    _ = self.expect(TokenType::LeftParanthesis)?;
    _ = self.expect(TokenType::RightParanthesis)?;

    // TODO: implement function parameters

    Ok(vec![])
  }

  fn parse_function(&mut self) -> Result<NodeIdx, String> {
    let tokidx = self.tokidx;

    _ = self.expect(TokenType::Defn)?;
    let name = self.expect(TokenType::Identifier)?;
    let _params = self.parse_parameter_declaration()?;
    _ = self.expect(TokenType::Colon)?;

    let exec = self.parse_block()?;

    // if the name is "main",
    // store into the pre-allocated 0 idx,

    if self.toks[name].slice == "main" {
      self.nodes[0] = Node {
        data: NodeData::FunctionDef(FunctionDef { name, exec }),
        tok: tokidx,
      };
      Ok(0)
    } else {
      Ok(self.push_node(Node {
        data: NodeData::FunctionDef(FunctionDef { name, exec }),
        tok: tokidx,
      }))
    }
  }

  // top level function declarations,
  // e.g. functions, global variables, import declarations
  fn parse_toplevel(&mut self) -> Result<(), String> {
    while let Token {
      ty: TokenType::Indentation,
      // only expect identation slices that are of indentation 0
      slice,
    } = self.next_tok()
    {
      if slice.len() != 0 {
        return Err(
          "Expected an indentation of level 0 when parsing top level declarations".to_string(),
        );
      };
      let func = self.parse_function()?;
      self.funcs.push(func);
    }

    Ok(())
  }

  pub fn parse(mut self) -> Result<Ast<'a>, String> {
    // reserve some space for the main function
    _ = self.push_node(Node {
      data: NodeData::Add(Binary { left: 0, right: 0 }),
      tok: 0,
    });

    self.parse_toplevel()?;

    Ok(Ast {
      toks: self.toks,
      nodes: self.nodes,
      funcs: self.funcs,
    })
  }
}

// impl<'a> Ast<'a> {
//   fn inner_display(&self, indentation: usize, out: &mut String) {
//     match self {

//     }
//   }
// }

// impl<'a> Display for Ast<'a> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     let out = m
//   }
// }

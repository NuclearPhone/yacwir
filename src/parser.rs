use crate::{
  context::CompilerContext,
  lexer::Lexer,
  node::{Binary, Node, NodeIdx, FunctionDef},
  token::{Token, TokenType},
};

pub struct Ast<'a> {
  pub toks: Vec<Token<'a>>,
  pub nodes: Vec<Node>,
}

pub struct Parser<'a> {
  ctx: &'a CompilerContext,
  nodes: Vec<Node>,
  toks: Vec<Token<'a>>,
  tokidx: usize,
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a CompilerContext) -> Result<Self, String> {
    println!("{:?}", Lexer::new(ctx).lex()?);
    
    Ok(Self {
      ctx,
      nodes: vec![],
      toks: Lexer::new(ctx).lex()?,
      tokidx: 0,
    })
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
          self.push_node(Node::Floating {
            val: slice
              .parse::<f64>()
              .ok()
              .ok_or("error while trying to parse a number".to_owned())?,

            tokidx: self.tokidx - 1,
          }),
        )
      }

      _ => Err("unknown symbol in parse_factor".into()),
    }
  }

  fn parse_term(&mut self) -> Result<NodeIdx, String> {
    let mut left = self.parse_factor()?;

    while let Some(Token {
      ty: TokenType::Asterisk | TokenType::Solidus,
      ..
    }) = self.current_tok()
    {
      let ty = self.current_tok().unwrap().ty;
      self.tokidx += 1;

      let right = self.parse_factor()?;

      left = match ty {
        TokenType::Asterisk => self.push_node(Node::Multiply(Binary { left, right })),
        TokenType::Solidus => self.push_node(Node::Divide(Binary { left, right })),

        _ => unreachable!(),
      }
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
      let ty = self.current_tok().unwrap().ty;
      self.tokidx += 1;

      let right = self.parse_term()?;

      left = match ty {
        TokenType::Plus => self.push_node(Node::Add(Binary { left, right })),
        TokenType::Minus => self.push_node(Node::Subtract(Binary { left, right })),

        _ => unreachable!(),
      }
    }

    Ok(left)
  }

  fn parse_block(&mut self) -> Result<NodeIdx, String> {
    // get indentation
    let Some(Token {ty: TokenType::Indentation, slice}) = self.current_tok() 
      else { return Err("Expected an indentation before parsing a block".into()) };

    let inden_len = slice.len();
    
    self.tokidx += 1;
    
    let mut toks = vec![];
    
    'l: loop {      
      toks.push(self.parse_expr()?);
      
      let new_ind: usize = match self.current_tok() {
        Some(Token{ty: TokenType::Indentation, slice}) => slice.len(),

        // break out of parsing loop on EOF
        Some(Token{ty: TokenType::EOF, ..}) => break 'l,

        _ => 
         return Err("expected an indentation while parsing block".into()),
      };

      
      if new_ind < inden_len { break }
      else if new_ind > inden_len {
        return Err("somehow tried to parse a larger indentation while parsing a block".into());
      }
      
      self.tokidx += 1;
    }
    
    Ok(self.push_node(Node::Block(toks)))
  }

  pub fn parse(mut self) -> Result<Ast<'a>, String> {
    // reserve some space for the main function
    _ = self.push_node(Node::Add(Binary { left: 0, right: 0 }));

    let main_block = self.parse_block()?;
    
    self.nodes[0] = Node::FunctionDef(FunctionDef {name: "main".into(), exec: main_block});
    
    Ok(Ast {
      toks: self.toks,
      nodes: self.nodes,
    })
  }
}

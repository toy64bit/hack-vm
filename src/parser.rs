use crate::ast::{Segment, Stmt};
use crate::token::{Token, TokenKind};

pub struct Parser {
  stmt_list: Vec<Stmt>,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      stmt_list: Vec::new(),
    }
  }

  pub fn parse(&mut self, tokens: &Vec<Vec<Token>>) -> Option<Vec<Stmt>> {
    for token_list in tokens {
      match token_list.first() {
        Some(op) => match op.kind {
          TokenKind::Push => {
            if token_list.len() != 3 {
              println!("invalid number of tokens found...");
            }
            let seg = token_list[1];
            let idx = token_list[2];
            let push_stmt = Stmt::Push {
              segment: self.parse_segment(seg).expect("invalid segment found"),
              index: self.parse_index(idx).expect("invalid index found"),
            };
            self.stmt_list.push(push_stmt)
          }
          TokenKind::Pop => {
            if token_list.len() != 3 {
              println!("invalid number of tokens found...");
            }
            let seg = token_list[1];
            let idx = token_list[2];
            let push_stmt = Stmt::Pop {
              segment: self.parse_segment(seg).expect("invalid segment found"),
              index: self.parse_index(idx).expect("invalid index found"),
            };
            self.stmt_list.push(push_stmt)
          }
          TokenKind::Add => self.stmt_list.push(Stmt::Add),
          TokenKind::Sub => self.stmt_list.push(Stmt::Add),
          _ => {
            println!("invalid token found as an operation...");
            return None;
          }
        },
        None => {
          println!("no tokens...");
          return None;
        }
      }
    }
    Some(self.stmt_list.clone())
  }

  fn parse_segment(&mut self, token: Token) -> Option<Segment> {
    match token.kind {
      TokenKind::Argment => Some(Segment::Argment),
      TokenKind::Local => Some(Segment::Local),
      TokenKind::Constatnt => Some(Segment::Constant),
      TokenKind::This => Some(Segment::This),
      TokenKind::That => Some(Segment::That),
      TokenKind::Pointer => Some(Segment::Temp),
      _ => None,
    }
  }

  fn parse_index(&mut self, token: Token) -> Option<i32> {
    match token.kind {
      TokenKind::Num(value) => Some(value),
      _ => None,
    }
  }
}

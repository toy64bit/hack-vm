#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
  Push,
  Pop,
  Print,

  Add,
  Sub,

  Argment,
  Local,
  Static,
  Constatnt,
  This,
  That,
  Pointer,
  Temp,

  Num(i32),
  Eof,
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
  pub kind: TokenKind,
}

use TokenKind::*;

// impl Token {
//   pub fn is_op(&self) -> bool {
//     match self.kind {
//       Push | Pop | Add | Sub | Mul => true,
//       _ => false,
//     }
//   }

//   pub fn is_segment(&self) -> bool {
//     match self.kind {
//       Argment | Local | Constatnt | This | That | Pointer | Temp => true,
//       _ => false,
//     }
//   }

//   pub fn is_num(&self) -> bool {
//     match self.kind {
//       Num(_) => true,
//       _ => false,
//     }
//   }
// }

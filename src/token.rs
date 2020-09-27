#[derive(Clone, Debug)]
pub enum TokenKind {
  Eof,
  Push,
  Pop,

  Argment,
  Local,
  Constatnt,
  This,
  That,
  Pointer,
  Temp,

  Add,
  Sub,
  Mul,

  Num(i32),
}

#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenKind,
}

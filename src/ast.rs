#[derive(Clone, Debug)]
pub enum Segment {
  Argment,
  Local,
  Constant,
  This,
  That,
  Pointer,
  Temp,
}

#[derive(Clone, Debug)]
pub enum Stmt {
  Push { segment: Segment, index: i32 },
  Pop { segment: Segment, index: i32 },
  Add,
  Sub,
}

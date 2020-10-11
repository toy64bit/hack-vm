#[derive(Clone, Debug)]
pub enum Segment {
  Argment,
  Local,
  Static,
  Constant,
  This,
  That,
  Pointer,
  Temp,
}

#[derive(Clone, Debug)]
pub enum DebugInfo {
  Stack,
  Memory { segment: Segment, index: i32 },
}

#[derive(Clone, Debug)]
pub enum Stmt {
  Push { segment: Segment, index: i32 },
  Pop { segment: Segment, index: i32 },
  Print { info: DebugInfo },
  Add,
  Sub,
}

use crate::token;

pub trait Node {
  // fn op(&self) -> Op;
  // fn lhs(&self) -> Box<Node>;
  // fn rhs(&self) -> Box<Node>;
}

pub struct Add {
  // kind:
  lhs: Box<Node>,
  rhs: Box<Node>,
}

impl Node for Add {}

// pub struct Node {
//   op: Op,
//   lhs: Box<Node>,
//   rhs: Box<Node>,
// }

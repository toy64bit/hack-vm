use std::io::prelude::*;
use std::io::BufWriter;

use crate::ast::{DebugInfo, Segment, Stmt};

const word_size: i32 = 4;
const constant_segment_size: i32 = 10;
const local_segment_size: i32 = 100;
const static_segment_size: i32 = 100;

pub struct Codegen<'a> {
  pub asm_list: Vec<String>,
  stmt_list: &'a Vec<Stmt>,
  indent: i32,
}

impl<'a> Codegen<'a> {
  pub fn new(stmt_list: &'a Vec<Stmt>) -> Self {
    Codegen {
      asm_list: Vec::new(),
      stmt_list: stmt_list,
      indent: 0,
    }
  }

  pub fn run(&mut self) {
    self.gen_init();

    self.gen_main_begin();
    for stmt in self.stmt_list {
      self.set(&format!("# {:?}", stmt));
      self.gen(stmt);
    }
    self.gen_main_end();
  }

  pub fn save(&mut self, file_name: &str) {
    let file_name = String::from(file_name) + file_name;
    let out_file = File::create(file_name)?;
    let mut writer = BufWriter::new(out_file);
    for line in self.asm_list {
      writer.write_all(line.as_bytes())?;
    }
    writer.flush()?;
  }

  pub fn gen_init(&mut self) {
    self.set(".section __DATA,__data");
    self.set("print_msg:");
    self.inc_indent();
    self.set(".asciz \"%d\\n\"");
    self.dec_indent();
    self.set("");

    // constant segment
    self.set("_constant:");
    self.inc_indent();
    for i in 0..constant_segment_size {
      self.set(&format!(".long {}", i));
    }
    self.dec_indent();
    self.set("");

    // local segment
    self.set(&format!(
      ".zerofill __DATA,__bss,_local,{},4",
      local_segment_size * word_size
    ));
    // local static
    self.set(&format!(
      ".zerofill __DATA,__bss,_static,{},4",
      static_segment_size * word_size
    ));
    self.set("");

    // TEXT section
    self.set(".section __TEXT,__text");
    self.set(".globl _main");
    self.set("");
  }

  pub fn gen_main_begin(&mut self) {
    self.set("_main:");
    self.inc_indent();
    self.set("push %rbp");
    self.set("mov %rsp, %rbp");
    self.set("# main start");
  }

  pub fn gen_main_end(&mut self) {
    self.set("# main end");
    self.set("mov $0, %rax");
    self.set("pop %rbp");
    self.set("ret");
    self.dec_indent();
  }

  fn gen(&mut self, stmt: &'a Stmt) {
    match stmt {
      Stmt::Push { segment, index } => {
        // String::new();
        let offset = index * word_size;
        let instr = match segment {
          Segment::Local => format!("push _local+{}(%rip)", offset),
          Segment::Constant => format!("push _constant+{}(%rip)", offset),
          _ => {
            panic!("not supported...");
          }
        };
        self.set(&instr);
      }

      Stmt::Pop { segment, index } => {
        let offset = index * word_size;
        let instr = match segment {
          Segment::Local => format!("pop _local+{}(%rip)", offset),
          Segment::Constant => format!("pop _constant+{}(%rip)", offset),
          _ => {
            panic!("not supported...");
          }
        };
        self.set(&instr);
      }

      Stmt::Print { info } => {
        self.set("mov print_msg@GOTPCREL(%rip), %rdi");
        match info {
          DebugInfo::Stack => {
            self.set("pop %rsi");
          }
          DebugInfo::Memory { segment, index } => {
            let offset = index * word_size;
            let src = match segment {
              Segment::Local => format!("_local+{}(%rip)", offset),
              Segment::Constant => format!("_constant+{}(%rip)", offset),
              _ => {
                panic!("not supported...");
              }
            };
            self.set(&format!("mov {}, %rsi", src));
          }
        }
        self.set("mov $0, %rax");
        self.set("call _printf");
      }

      Stmt::Add => {
        self.set("pop %rax");
        self.set("pop %rbx");
        self.set("add %rbx, %rax");
        self.set("push %rax");
      }

      Stmt::Sub => {
        self.set("pop %rax");
        self.set("pop %rbx");
        self.set("sub %rbx, %rax");
        self.set("push %rax");
      }
    }
  }

  fn set(&mut self, cmd: &str) {
    let mut indent = String::new();
    for _ in 0..self.indent {
      indent += " ";
    }
    self.asm_list.push(format!("{}{}\n", indent, cmd));
  }

  fn inc_indent(&mut self) {
    self.indent += 2;
  }

  fn dec_indent(&mut self) {
    self.indent -= 2;
  }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::token::{Token, TokenKind};

pub struct Scanner<R> {
  reader: R,
  position: usize,
  line_num: usize,
  current_line: String,
  token_map: HashMap<String, Token>,
  tokens: Vec<Token>,
}

impl<R: BufRead> Scanner<R> {
  pub fn new(reader: R) -> Scanner<R> {
    let mut map = HashMap::new();
    map.insert(
      String::from("push"),
      Token {
        kind: TokenKind::Push,
      },
    );
    map.insert(
      String::from("pop"),
      Token {
        kind: TokenKind::Pop,
      },
    );
    map.insert(
      String::from("constant"),
      Token {
        kind: TokenKind::Constatnt,
      },
    );
    map.insert(
      String::from("add"),
      Token {
        kind: TokenKind::Add,
      },
    );

    Scanner {
      reader: reader,
      position: 0,
      line_num: 0,
      current_line: String::new(),
      token_map: map,
      tokens: Vec::new(),
    }
  }

  pub fn scan(&mut self) -> Result<Vec<Token>, &str> {
    loop {
      let mut line = String::new();
      let res = self.reader.read_line(&mut line);

      match res {
        Ok(0) => break,
        Ok(_) => {
          self.line_num += 1;
          let l = line.trim().to_string();
          if line.starts_with("//") {
            continue;
          }
          self.current_line = l;
          self.parse_line();
          self.reset_position();
        }
        Err(..) => return Err("unexpected error..."),
      }
    }

    Ok(self.tokens.clone())
  }

  pub fn parse_line(&mut self) -> Result<(), &str> {
    let mut word = String::new();
    let mut word_list = Vec::new();

    while !self.is_eof() {
      if self.is_ignorable() {
        if word.len() > 0 {
          word_list.push(word);
          word = String::new();
        }
        self.consume();
      } else {
        let c = self.current().unwrap();
        word.push(c);
        self.consume();
      }
    }
    // push last word
    if word.len() > 0 {
      word_list.push(word);
    }

    let is_num = Regex::new(r"\d+").unwrap();
    let is_alpha = Regex::new(r"([a-z]|[A-Z])+").unwrap();

    for word in word_list {
      if is_num.is_match(&word) {
        let num = word.parse::<i32>().unwrap();
        self.tokens.push(Token {
          kind: TokenKind::Num(num),
        });
      }
      if is_alpha.is_match(&word) {
        match self.token_map.get(&word) {
          Some(tok) => {
            self.tokens.push(tok.clone());
          }
          None => {}
        }
      }

      self.consume();
    }

    Ok(())
  }

  fn parse_num(&mut self) -> Option<u32> {
    let mut i = 0;
    let mut num = 0;

    loop {
      match self.current().unwrap().to_digit(10) {
        Some(d) => {
          num += d * i;
          i *= 10;
          self.position += 1;
        }
        None => {
          if i == 0 {
            return Some(num);
          } else {
            return None;
          }
        }
      }
    }
  }

  fn current(&mut self) -> Option<char> {
    self.current_line.chars().nth(self.position)
  }

  fn last(&mut self) -> Option<char> {
    self.current_line.chars().nth(self.position - 1)
  }

  fn next(&mut self) -> Option<char> {
    self.current_line.chars().nth(self.position + 1)
  }

  fn consume(&mut self) {
    self.position += 1;
  }

  fn reset_position(&mut self) {
    self.position = 0;
  }

  fn is_ignorable(&mut self) -> bool {
    if self.is_eof() {
      return false;
    }
    let c = self.current().unwrap();
    c == ' ' || c == '\n' || c == '\t'
  }

  fn is_eof(&mut self) -> bool {
    match self.current() {
      Some(_) => false,
      None => true,
    }
  }
}

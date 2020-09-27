mod ast;
mod scanner;
mod token;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use scanner::Scanner;

fn main() -> std::io::Result<()> {
    println!("Hack VM");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("arguments must be 2...");
    }

    let file_path = &args[1];
    // let file_path = String::from("resource/c/test1.c");
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut scanner = Scanner::new(&mut reader);

    let tokens = scanner.scan().expect("something happened...");
    println!("{:?}", tokens);
    // scanner.build();

    // let path = if let Some(p) = output {
    //     PathBuf::from(p.as_ref())
    // } else {
    //     path.as_ref().with_extension("rs")
    // };

    // let file = File::create(path)?;
    // let mut writer = BufWriter::new(file);
    // scanner.generate(&mut writer)?;
    Ok(())
}

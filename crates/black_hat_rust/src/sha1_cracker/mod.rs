use std::{
  error::Error,
  fs::File,
  io::{BufRead, BufReader},
};

use sha1::Digest;

/// Simple SHA1 cracker

const TARGET: &str = "7110eda4d09e062aa5e4a390b0a572ac0d2c0220";

const WORD_LIST_PATH: &str = "./crates/black_hat_rust/src/sha1_cracker/word_list.txt";

fn compare(str: &str) -> bool {
  hex::encode(sha1::Sha1::digest(str.as_bytes())) == TARGET
}

fn read_word_list() -> Result<BufReader<File>, Box<dyn Error>> {
  let path = std::env::current_dir()?.join(WORD_LIST_PATH);
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  Ok(reader)
}

#[allow(dead_code)]
fn cracker_2() -> Result<(), Box<dyn Error>> {
  // This will bundle the file into the binary, so path should only be string literal
  let file = include_str!("./word_list.txt");
  file.lines().any(|s| {
    println!("{s}");
    if compare(s) {
      println!("Found: {}", s);
      return true;
    }
    false
  });
  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  /*
     RAII: Resource Acquisition Is Initialization
     Will free word_list.txt file handle when it goes out of scope
  */
  for line in read_word_list()?.lines() {
    let str = line?.trim().to_string();
    if compare(&str) {
      println!("Found: {}", str);
      return Ok(());
    }
  }
  println!("Not found");
  Ok(())
}

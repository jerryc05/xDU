#![feature(exact_size_is_empty)]

mod parse_args;
mod xdu;

fn main() {
  let config = parse_args::parse();
  println!("{:?}", config);
}

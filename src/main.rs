use std::{env, process::exit};

mod math;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    print_usage();
    exit(-1);
  }

  match args[1].parse::<u32>() {
    Ok(numator) => match args[2].parse::<u32>() {
      Ok(denominator) => {
        let result = math::Ratio::from((numator, denominator));
        println!("{}", result);
      }
      Err(_) => {
        eprintln!("The denominator must be a positive integer.");
        print_usage();
        exit(-1);
      }
    },
    Err(_) => {
      eprintln!("The numerator must be a positive integer.");
      print_usage();
      exit(-1);
    }
  }
}

fn print_usage() {
  eprintln!("Usage: ratio <numerator> <denominator>");
}

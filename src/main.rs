
// in rust, it is customary to structure programs around the data 
// they handle
//
use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");
    
    for line in content.lines() {
      if line.contains(&args.pattern) {
        println!("{}", line);
      }
    }
}


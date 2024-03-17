
// in rust, it is customary to structure programs around the data 
// they handle
// I wasn't able to use the BufReader to read the file from the cli
use clap::Parser;
use anyhow::{Context, Result};
// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
  /// the pattern to look for
  pattern: String,
  /// the path to the file to read
  path: std::path::PathBuf
}


// A couple of action points
// Functions are testable when they are smaller
// so since we cannot call the main function,
// we break them down into smaller functions
fn main() -> Result<()> { 
    let args = Cli::parse();
    
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    
    grrk::find_matches(&content, &args.pattern, &mut std::io::stdout());
  Ok(())
}






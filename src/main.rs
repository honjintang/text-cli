use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let num_of_lines = content.lines().count();

    let mut word_count = 0;
    let mut chars_count = 0;
    for line in content.lines() {
        word_count += line.split_whitespace().count();
        chars_count += line.chars().count();
    }

    println!("number of lines: {:?}, number of words: {:?}, number of characters: {:?}", num_of_lines, word_count, chars_count);

    Ok(())
}

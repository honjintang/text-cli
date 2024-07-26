use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).expect("could not read file");

    let num_of_lines = content.lines().count();

    println!("number of lines: {:?}", num_of_lines);
}

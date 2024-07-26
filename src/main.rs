use clap::Parser;

#[derive(Parser)]
struct Cli {
    file_name: String,
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    println!("file name: {:?}, path: {:?}", args.file_name, args.path);
}

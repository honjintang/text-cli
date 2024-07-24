struct Cli {
    file_name: String,
    path: std::path::PathBuf
}
fn main() {
    let file_name = std::env::args().nth(1).expect("no file_name given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        file_name,
        path: std::path::PathBuf::from(path)
    };

    println!("file name: {:?}, path: {:?}", args.file_name, args.path);
}

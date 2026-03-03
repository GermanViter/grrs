use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    generateLines(pattern, path);
}

fn generateLines(pattern: String, path: String) {
    let args = Cli::parse();
    let reader = std::fs::read_to_string(path).expect("could not read file");

    for line in reader.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

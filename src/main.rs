use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    //generate_lines(pattern, path);
    out_put_file(path, pattern);
}

//fn generate_lines(pattern: String, path: String) {
//    let args = Cli::parse();
//    let reader = std::fs::read_to_string(path).expect("could not read file");
//
//    for line in reader.lines() {
//        if line.contains(&args.pattern) {
//            println!("{}", line);
//        }
//    }
//}

fn out_put_file(path: String, pattern: String) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line_content = line?;
        if line_content.contains(&pattern) {
            println!("{line_content}");
        }
    }

    Ok(())
}

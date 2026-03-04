use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;


#[derive(Parser)]
#[derive(Debug)]

struct CustomError(String);

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    generate_lines(pattern, path);
    //out_put_file(path, pattern);
}

fn generate_lines(pattern: String, path: String) -> Result<(), CustomError> {
   let args = Cli::parse();
   let reader = std::fs::read_to_string(path)
       .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;

   println!("file content: {:?}", reader);
   Ok(())
}

//fn out_put_file(path: String, pattern: String) -> std::io::Result<()> {
//    let file = File::open(path)?;
//    let reader = BufReader::new(file);
//    
//    for line in reader.lines() {
//        let line_content = line?;
//        if line_content.contains(&pattern) {
//            match line_content {
//                Ok(content) => { println!("Result: {content}"); }
//                Err(error) => { println!("oh noes: {error}"); }
//            }
//        }
//    }
//
//    Ok(())
//}


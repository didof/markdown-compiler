use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn usage() {
    println!("Usage: markdown-compiler <your_file>.md");
}

fn print_short_banner() {
    println!("{}", get_title());
}

// fn print_long_banner() {
//     print_short_banner();
//     println!("Written by {}", env!("CARGO_PKG_AUTHORS"));
//     println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
//     usage();
// }

fn get_title() -> String {
    let mut title: String = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Starting parser on file {}", filename);

    let input_filename = Path::new(filename);

    // let file = match File::open(&input_filename) {
    //     Ok(value) => value,
    //     Err(error) => {
    //         println!("[ ERROR ] Failed to open {}", input_filename.display());
    //         println!("[ REASON ] {}", error);
    //         panic!("parse_markdown_file");
    //     }
    // };

    let file = File::open(&input_filename).expect("[ ERROR ] Failder file opening.");

    let mut _htag = false;
    let mut _ptag = false;
    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();

        let mut output_line = String::new();

        let mut first_char: Vec<char> = line_content.chars().take(1).collect();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_content[2..]);
            }
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_content);
            }
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");
    let mut outfile = File::create(output_filename).expect("[ ERROR ] Failed file creation.");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Failed line writing in file.");
    }

    println!("[ INFO ] Parsing complete!");
}

fn main() {
    print_short_banner();

    let args: std::env::Args = std::env::args();
    let args: Vec<String> = args.collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => usage(),
    }
}

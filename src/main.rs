/*use anyhow::anyhow;
use svg_file_parser::*;

fn get_content_from_file(name: &str) -> String {
    let path = format!("example/{}", name);
    return std::fs::read_to_string(path).expect("Unable to read file");
}

fn main() {
   // Приклад рядка для парсингу
   let input = r#"<svg>
           <circle cx="10" cy="10" r="5" />
           <rect x="20" y="20" width="10" height="10" />
           Some text content
       </svg>
   "#;

   //let input = get_content_from_file("example_correct.svg");
   match parse_svg(&input) {
       Ok(result) => {
           println!("Parsing successful! Result: {:#?}", result);
       }
       Err(error) => {
           eprintln!("Error during parsing: {:?}", error);
       }
   }
}*/

extern crate pest;
extern crate pest_derive;

use crate::cli::{Cli, Commands};
use clap::Parser;
use svg_file_parser::parse_svg;
mod cli;

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { input, output }) => {
            if let Some(path_to_file) = input {
                let content = std::fs::read_to_string(path_to_file).expect("Unable to read file");
                let parsed_svg = parse_svg(&content);
                let result: String = match parsed_svg {
                    Ok(svg) => {
                        let mut result_string = String::new();
                        for svg_content in svg {
                            result_string += &svg_content.to_string();
                        }
                        result_string
                    }
                    Err(e) => e.to_string(),
                };
                if let Some(output_path) = output {
                    std::fs::write(output_path, result).expect("Unable to write result");
                } else {
                    println!("{}", result);
                }
            } else {
                println!("Missing path to file!");
            }
        }
        None => {}
    }
}

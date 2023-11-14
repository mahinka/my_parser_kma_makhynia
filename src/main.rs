use anyhow::anyhow;
use svg_file_parser::*;

fn get_content_from_file(name: &str) -> String {
    let path = format!("example/{}", name);
    return std::fs::read_to_string(path).expect("Unable to read file");
}

fn main() {
   // Приклад рядка для парсингу
   /*let input = r#"<svg>
           <circle cx="10" cy="10" r="5" />
           <rect x="20" y="20" width="10" height="10" />
           Some text content
       </svg>
   "#;*/

   let input = get_content_from_file("example_correct.svg");
   match parse_svg(&input) {
       Ok(result) => {
           println!("Parsing successful! Result: {:#?}", result);
       }
       Err(error) => {
           eprintln!("Error during parsing: {:?}", error);
       }
   }
}





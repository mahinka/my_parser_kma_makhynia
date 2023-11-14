use anyhow::anyhow;
use my_parser_kma_makhynia::*;
fn main() {
   // Приклад рядка для парсингу
   let input = r#"<svg>
           <circle cx="10" cy="10" r="5" />
           <rect x="20" y="20" width="10" height="10" />
           Some text content
       </svg>
   "#;

   match parse_svg(input) {
       Ok(result) => {
           println!("Parsing successful! Result: {:#?}", result);
       }
       Err(error) => {
           eprintln!("Error during parsing: {:?}", error);
       }
   }
}



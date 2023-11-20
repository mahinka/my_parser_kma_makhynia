use pest::{Parser, Span};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "./svg.pest"]
#[doc = r"Struct for parsing svg file"]
pub struct SvgParser;

#[derive(Debug, PartialEq, Eq)]
#[doc = r"Enum for svg file content"]
pub enum SvgContent {
    Svg(Vec<(String, String)>),
    Circle(Vec<(String, String)>),
    Ellipse(Vec<(String, String)>),
    Rect(Vec<(String, String)>),
    Line(Vec<(String, String)>),
    Text(String),
}

impl ToString for SvgContent {
    fn to_string(&self) -> String {
        match self {
            SvgContent::Svg(attributes) => format!("Svg({})\n", stringify_attributes(attributes)),
            SvgContent::Circle(attributes) => {
                format!("Circle({})\n", stringify_attributes(attributes))
            }
            SvgContent::Ellipse(attributes) => {
                format!("Ellipse({})\n", stringify_attributes(attributes))
            }
            SvgContent::Rect(attributes) => format!("Rect({})\n", stringify_attributes(attributes)),
            SvgContent::Line(attributes) => format!("Line({})\n", stringify_attributes(attributes)),
            SvgContent::Text(text) => format!("Text({})\n", text),
        }
    }
}

fn stringify_attributes(attributes: &[(String, String)]) -> String {
    attributes
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<String>>()
        .join(", ")
}

#[doc = r"Parse a svg file and return a vector of SvgContent"]
pub fn parse_svg(input: &str) -> Result<Vec<SvgContent>, Box<pest::error::Error<Rule>>> {
    let mut tokens = vec![];
    let parse_result = SvgParser::parse(Rule::svg_file, input);

    match parse_result {
        Ok(pairs) => {
            let svg_file = pairs
                .into_iter()
                .find(|pair| pair.as_rule() == Rule::svg_file)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No SVG file found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            let svg_open = svg_file
                .clone()
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::svg_open)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No SVG open tag found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;
            let mut attributes: Vec<(String, String)> = Vec::new();
            for attribute_pair in svg_open.into_inner() {
                if attribute_pair.as_rule() == Rule::attribute {
                    let mut attribute_pair_iter = attribute_pair.clone().into_inner();
                    let attribute_name = attribute_pair_iter.next().unwrap().as_str().to_string();
                    let attribute_value = attribute_pair_iter.next().unwrap().as_str().to_string();
                    attributes.push((
                        attribute_name.as_str().to_string(),
                        attribute_value.as_str().to_string(),
                    ));
                }
            }
            tokens.push(SvgContent::Svg(attributes));

            let svg_content = svg_file
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::svg_content)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No SVG content found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            for inner_pair in svg_content.into_inner() {
                match inner_pair.as_rule() {
                    Rule::element => {
                        let element = inner_pair.into_inner().next().unwrap();
                        match element.as_rule() {
                            Rule::circle => {
                                let mut attributes: Vec<(String, String)> = Vec::new();
                                for attributes_pair in element.into_inner() {
                                    if attributes_pair.as_rule() == Rule::attribute {
                                        let mut attributes_pair_iter =
                                            attributes_pair.clone().into_inner();
                                        let attribute_name = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        let attribute_value = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        attributes.push((
                                            attribute_name.as_str().to_string(),
                                            attribute_value.as_str().to_string(),
                                        ));
                                    }
                                }
                                tokens.push(SvgContent::Circle(attributes));
                            }
                            Rule::ellipse => {
                                let mut attributes: Vec<(String, String)> = Vec::new();
                                for attributes_pair in element.into_inner() {
                                    if attributes_pair.as_rule() == Rule::attribute {
                                        let mut attributes_pair_iter =
                                            attributes_pair.clone().into_inner();
                                        let attribute_name = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        let attribute_value = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        attributes.push((
                                            attribute_name.as_str().to_string(),
                                            attribute_value.as_str().to_string(),
                                        ));
                                    }
                                }
                                tokens.push(SvgContent::Ellipse(attributes));
                            }
                            Rule::rect => {
                                let mut attributes: Vec<(String, String)> = Vec::new();
                                for attributes_pair in element.into_inner() {
                                    if attributes_pair.as_rule() == Rule::attribute {
                                        let mut attributes_pair_iter =
                                            attributes_pair.clone().into_inner();
                                        let attribute_name = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        let attribute_value = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        attributes.push((
                                            attribute_name.as_str().to_string(),
                                            attribute_value.as_str().to_string(),
                                        ));
                                    }
                                }
                                tokens.push(SvgContent::Rect(attributes));
                            }
                            Rule::line => {
                                let mut attributes: Vec<(String, String)> = Vec::new();
                                for attributes_pair in element.into_inner() {
                                    if attributes_pair.as_rule() == Rule::attribute {
                                        let mut attributes_pair_iter =
                                            attributes_pair.clone().into_inner();
                                        let attribute_name = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        let attribute_value = attributes_pair_iter
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .to_string();
                                        attributes.push((
                                            attribute_name.as_str().to_string(),
                                            attribute_value.as_str().to_string(),
                                        ));
                                    }
                                }
                                tokens.push(SvgContent::Line(attributes));
                            }
                            _ => {}
                        }
                    }
                    Rule::text_content => {
                        tokens.push(SvgContent::Text(inner_pair.as_str().to_string()));
                    }
                    _ => {}
                }
            }

            Ok(tokens)
        }
        Err(error) => Err(Box::new(error)),
    }
}

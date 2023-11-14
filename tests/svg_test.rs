use svg_file_parser::*;
use pest::Parser;

fn get_content_from_file(name: &str) -> String {
    let path = format!("example/{}", name);
    return std::fs::read_to_string(path).expect("Unable to read file");
}

#[test]
fn svg_file_rule_test() -> anyhow::Result< () > {
    let input = get_content_from_file("svg_file_rule_correct.svg");
    let result = SvgParser::parse(Rule::svg_file, &input);
    assert!( result.is_ok() );

    let input = get_content_from_file("svg_file_rule_uncorrect.svg");
    let result = SvgParser::parse(Rule::svg_file, &input);
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn svg_open_rule_test() -> anyhow::Result< () > {
    
    let result = SvgParser::parse(Rule::svg_open, "<svg>\n");
    assert!( result.is_ok() );

    let input = get_content_from_file("svg_open_rule_correct.svg");
    let result = SvgParser::parse(Rule::svg_open, &input);
    assert!( result.is_ok() );

    let input = get_content_from_file("svg_open_rule_uncorrect.svg");
    let result = SvgParser::parse(Rule::svg_open, &input);
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn svg_content_rule_test() -> anyhow::Result< () > {
    
    let result = SvgParser::parse(Rule::svg_content, "Hello,world\n");
    assert!( result.is_ok() );

    Ok( () )
}

#[test]
fn svg_close_rule_test() -> anyhow::Result< () > {
    
    let result = SvgParser::parse(Rule::svg_close, "</svg>");
    assert!( result.is_ok() );

    let result = SvgParser::parse(Rule::svg_close, "<svg>");
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn element_rule_test() -> anyhow::Result< () > {
    
    let input = get_content_from_file("element_rule_correct.svg");
    let result = SvgParser::parse(Rule::element, &input);
    assert!( result.is_ok() );

    Ok( () )
}

#[test]
fn circle_rule_test() -> anyhow::Result< () > {
    
    let input = get_content_from_file("element_rule_correct.svg");
    let result = SvgParser::parse(Rule::circle, &input);
    assert!( result.is_ok() );

    let result = SvgParser::parse(Rule::circle, "<circle/>");
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn rect_rule_test() -> anyhow::Result< () > {
    
    let input = get_content_from_file("rect_rule_correct.svg");
    let result = SvgParser::parse(Rule::rect, &input);
    assert!( result.is_ok() );

    let result = SvgParser::parse(Rule::rect, "<rect/>");
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn line_rule_test() -> anyhow::Result< () > {
    
    let input = get_content_from_file("line_rule_correct.svg");
    let result = SvgParser::parse(Rule::line, &input);
    assert!( result.is_ok() );

    let result = SvgParser::parse(Rule::line, "<line/>");
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn ellipse_rule_test() -> anyhow::Result< () > {
    
    let input = get_content_from_file("ellipse_rule_correct.svg");
    let result = SvgParser::parse(Rule::ellipse, &input);
    assert!( result.is_ok() );

    let result = SvgParser::parse(Rule::ellipse, "<ellipse/>");
    assert!( result.is_err() );

    Ok( () )
}

#[test]
fn attribute_rule_test() -> anyhow::Result< () > {

    let input = r#"x="10""#;
    let result = SvgParser::parse(Rule::attribute, &input);
    assert!( result.is_ok() );

    let input = "x=10";
    let result = SvgParser::parse(Rule::attribute, &input);
    assert!( result.is_err() );

    let input = "=";
    let result = SvgParser::parse(Rule::attribute, &input);
    assert!( result.is_err() );

    let input = "d=";
    let result = SvgParser::parse(Rule::attribute, &input);
    assert!( result.is_err() );

    Ok( () )
}

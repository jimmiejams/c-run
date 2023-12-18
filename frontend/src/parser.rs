use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "jc.pest"]
pub struct JcParser;

impl JcParser {
    pub fn parse_input_file(input_filename: &str)  {
        let unparsed_file = fs::read_to_string(input_filename)
            .unwrap_or_else(|e| { panic!("cannot read file: {}", e)});
        let p = JcParser::parse(Rule::translation_unit, &unparsed_file);
        match p {
            Ok(_) => println!("file parsed correctly"),
            Err(e) => panic!("parse error: {}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let p = JcParser::parse(Rule::translation_unit, "fn main() {}");
        assert_eq!(p.is_ok(), true);
    }
}
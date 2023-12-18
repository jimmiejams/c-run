use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "jc.pest"]
pub struct JcParser {
    unparsed_input_file: String,
}

impl JcParser {
    pub fn new() -> JcParser {
        JcParser {
            unparsed_input_file: String::from(""),
        }
    }

    pub fn parse_input_file(&mut self, input_filename: &str) -> pest::iterators::Pairs<Rule>  {
        self.unparsed_input_file = fs::read_to_string(input_filename)
            .unwrap_or_else(|e| { panic!("cannot read file: {}", e)});
        let p = JcParser::parse(Rule::translation_unit, &self.unparsed_input_file)
            .unwrap_or_else(|e| { panic!("parse error: {}", e); });
        p
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
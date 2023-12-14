use pest::Parser;
use pest_derive::Parser;

pub mod parser {
    #[derive(pest_derive::Parser)]
    #[grammar = "jc.pest"]
    struct JcParser;
}
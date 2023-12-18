use clap::Parser;
use frontend::parser::{JcParser};

#[derive(Parser, Debug)]
struct DriverCli {
    filename: String,
}

fn main() {
    let args = DriverCli::parse();
    let mut parser = JcParser::new();
    let ast = parser.parse_input_file(&args.filename);
    println!("{:?}", ast);
}

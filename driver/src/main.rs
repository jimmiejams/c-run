use clap::Parser;
use frontend::parser::{JcParser};

#[derive(Parser, Debug)]
struct DriverCli {
    filename: String,
}

fn main() {
    let args = DriverCli::parse();
    JcParser::parse_input_file(&args.filename);
}

#[macro_use]
extern crate serde_derive;

pub mod parser;

use parser::dialect::Dialect;
use parser::io::InputStream;
use parser::parser::GherkinParser;
use std::fs;


fn main() {
    let dialect = Dialect::dialect_of(&"en");
    let file_name: &str = "features/my_first.feature";
    let mut lang_file = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let input_stream: InputStream = InputStream::from_string(&mut lang_file);
    println!("{:?}", &dialect);
    let parser = GherkinParser::create(dialect, input_stream);
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "aql.pest"]
pub struct AQLParser;

fn main() {
    println!("Hello, world!");
}

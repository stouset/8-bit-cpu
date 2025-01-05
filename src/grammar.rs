use core::result::Result;

use pest::Parser;
use pest_derive::Parser;

pub use pest::Span;
pub use pest::iterators::{Pair, Pairs};
pub use pest::error::Error;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
struct Grammar;

pub fn parse(program: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    Grammar::parse(Rule::program, program)
}

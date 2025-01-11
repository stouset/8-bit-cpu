mod directive;
mod instruction;

pub use self::directive::*;
pub use self::instruction::*;

use crate::grammar::{Pairs, Pair, Rule};
use crate::error::{Result, SyntaxError};

pub fn consume(pairs: Pairs<'_, Rule>) -> Result<Ast, SyntaxError> {
    pairs.try_into()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address([u8; 1]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Word([u8; 1]);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label(String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ast {
    statements: Vec<Statement>
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Statement {
    Directive(Directive),
    Instruction(Instruction),
    EOI,
}

impl TryFrom<Pair<'_, Rule>> for Word {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let number = match pair.as_str().as_bytes() {
            [b'-', ..] => str::parse::<i8>(pair.as_str()).map(i8::to_le_bytes),
            [..]       => str::parse::<u8>(pair.as_str()).map(u8::to_le_bytes),
        };

        number
            .map(Self)
            .map_err(|e| SyntaxError::new_from_pair(e, pair))
    }
}

impl TryFrom<Pair<'_, Rule>> for Label {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(Self(pair.as_str().to_string()))
    }
}

impl TryFrom<Pairs<'_, Rule>> for Ast {
    type Error = SyntaxError;

    fn try_from(pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let statements = pairs
            .map(Statement::try_from)
            .filter(|r| r.as_ref().is_ok_and(|s| *s != Statement::EOI))
            .collect::<Result<Vec<_>, Self::Error>>()?;

        Ok(Self { statements })
    }
}

impl TryFrom<Pair<'_, Rule>> for Statement {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let statement = match pair.as_rule() {
            Rule::directive   => Statement::Directive(Directive::try_from(pair)?),
            Rule::instruction => Statement::Instruction(Instruction::try_from(pair)?),
            Rule::EOI         => Statement::EOI,

            _ => unreachable!("pext grammar produced a statement which was neither a directive nor instruction"),
        };

        Ok(statement)
    }
}

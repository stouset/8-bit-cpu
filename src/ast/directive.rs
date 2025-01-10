use crate::ast::{Label, Word};
use crate::error::{Result, SyntaxError, SyntaxErrorType};
use crate::grammar::{Pair, Rule};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Directive {
    command:   Command,
    arguments: Vec<Argument>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    EQU,
    ORG,
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Argument {
    Label(Label),
    Word(Word),
}

impl TryFrom<Pair<'_, Rule>> for Directive {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner();

        let command = inner.next()
            .unwrap_or_else(|| unreachable!("pext grammar produced an insrtuction without a mnemonic"))
            .try_into()?;

        let arguments = inner.next()
            .unwrap_or_else(|| unreachable!("pext grammar produced an insrtuction without a mnemonic"))
            .into_inner()
            .map(Argument::try_from)
            .collect::<Result<Vec<_>, Self::Error>>()?;

        Ok(Self { command, arguments })
    }
}

impl TryFrom<Pair<'_, Rule>> for Command {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let command = match pair.as_str().to_lowercase().as_str() {
            "equ" => Command::EQU,
            "org" => Command::ORG,

            s => SyntaxError::err_from_pair(SyntaxErrorType::CommandUnknown(s.to_string()), pair)?,
        };

        Ok(command)
    }
}

impl TryFrom<Pair<'_, Rule>> for Argument {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value = match pair.as_rule() {
            Rule::label    => Self::Label(Label::try_from(pair)?),
            Rule::number   => Self::Word(Word::try_from(pair)?),

            _ => unreachable!("pest grammar produced an unknown target"),
        };

        Ok(value)
    }
}

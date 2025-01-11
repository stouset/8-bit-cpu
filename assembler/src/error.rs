use crate::grammar::{Pair, Rule, Span};

use thiserror::Error;

pub type Error                = color_eyre::Report;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("[{}:{} - {}:{}] {line} ; {ty}", span.0.0, span.0.1, span.1.0, span.1.1)]
pub struct SyntaxError {
    pub ty:      SyntaxErrorType,
    pub line:    String,
    pub span:    ((usize, usize), (usize, usize)),
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SyntaxErrorType {
    #[error("command {0} is not recognized")]
    CommandUnknown(String),

    #[error("mnemonic {0} is not recognized")]
    MnemonicUnknown(String),

    #[error("register {0} is not defined on this architecture")]
    RegisterUnknown(String),

    #[error("{name} expects {expect} arguments but {given} were given")]
    Arguments { name: String, expect: usize, given: usize },

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{0}")]
    Custom(String),
}

impl SyntaxError {
    #[must_use]
    pub fn new_from_span<E: Into<SyntaxErrorType>>(ty: E, span: Span<'_>) -> Self {
        Self {
            ty: ty.into(),
            line: span.as_str().into(),
            span: (span.start_pos().line_col(), span.end_pos().line_col()),
        }
    }

    #[must_use]
    pub fn new_from_pair<E: Into<SyntaxErrorType>>(ty: E, pair: Pair<'_, Rule>) -> Self {
        Self::new_from_span(ty, pair.as_span())
    }

    #[must_use]
    pub fn err_from_span<T, E: Into<SyntaxErrorType>>(ty: E, span: Span<'_>) -> Result<T, Self> {
        Err(Self::new_from_span(ty, span))
    }

    #[must_use]
    pub fn err_from_pair<T, E: Into<SyntaxErrorType>>(ty: E, pair: Pair<'_, Rule>) -> Result<T, Self> {
        Err(Self::new_from_span(ty, pair.as_span()))
    }
}

// #[derive(Error, Debug)]
// pub enum SyntaxError {
//     #[error("unexpected token")]
//     Unexpected,

//     #[error("directive missing")]
//     DirectiveMissing,

//     #[error("directive {0} is unknown")]
//     DirectiveUnknown(String),

//     #[error("directive expects {expected} arguments, but {provided} were provided")]
//     DirectiveArguments { expected: usize, provided: usize },

//     #[error("mnemonic missing")]
//     MnemonicMissing,

//     #[error("mnemonic {0} unknown")]
//     MnemonicUnknown(String),

//     #[error("mnemonic expects {expected} arguments, but {provided} were provided")]
//     MnemonicArguments { expected: usize, provided: usize },

//     #[error("register {0} unknown")]
//     RegisterUnknown(String),

//     #[error("unable to parse token as a byte")]
//     ParseByteError(#[from] ParseIntError),

//     #[error("statement was not an instruction nor a directive")]
//     StatementUnknown,
// }

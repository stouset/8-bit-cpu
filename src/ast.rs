use crate::grammar::{Pairs, Pair, Rule};
use crate::error::{Result, SyntaxError, SyntaxErrorType};

pub fn consume(pairs: Pairs<'_, Rule>) -> Result<Ast, SyntaxError> {
    pairs.try_into()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address([u8; 1]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Word([u8; 1]);

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


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label(String);

impl TryFrom<Pair<'_, Rule>> for Label {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(Self(pair.as_str().to_string()))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ast {
    statements: Vec<Statement>
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Directive {
    command:   Command,
    arguments: Vec<Argument>,
}

impl TryFrom<Pair<'_, Rule>> for Directive {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner().peekable();

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    EQU,
    ORG,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]

pub enum Argument {
    Label(Label),
    Word(Word),
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


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction {
    label:    Option<Label>,
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

impl TryFrom<Pair<'_, Rule>> for Instruction {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner().peekable();
        let     peek  = inner.peek().unwrap_or_else(|| unreachable!("pext grammar produced an empty instruction"));

        let label = match peek.as_rule() {
            Rule::label    => Some(Label::try_from(inner.next().unwrap())?),
            Rule::mnemonic => None,

            _ => unreachable!("pext grammar produced an instruction without a label or mnemonic"),
        };

        let m  = inner.next().unwrap_or_else(|| unreachable!("pext grammar produced an insrtuction without a mnemonic"));
        let os = inner.next().unwrap_or_else(|| unreachable!("pext grammar produced an insrtuction without operands"));

        let mnemonic = Mnemonic::try_from(m)?;
        let operands = os.into_inner()
            .map(Operand::try_from)
            .collect::<Result<Vec<_>, Self::Error>>()?;

        let instruction = Self {
            label,
            mnemonic,
            operands,
        };

        Ok(instruction)
    }
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mnemonic {
    ADD,
    CALL,
    CMP,
    DATA,
    HLT,
    INC,
    JC,
    JN,
    JO,
    JZ,
    JMP,
    LOD,
    MOV,
    POP,
    PUSH,
    RET,
    STO,
    SUB,
    TST,
    XOR,
}

impl TryFrom<Pair<'_, Rule>> for Mnemonic {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mnemonic = match pair.as_str().to_lowercase().as_str() {
            "add"  => Mnemonic::ADD,
            "call" => Mnemonic::CALL,
            "cmp"  => Mnemonic::CMP,
            "data" => Mnemonic::DATA,
            "hlt"  => Mnemonic::HLT,
            "inc"  => Mnemonic::INC,
            "jc"   => Mnemonic::JC,
            "jn"   => Mnemonic::JN,
            "jo"   => Mnemonic::JO,
            "jz"   => Mnemonic::JZ,
            "jmp"  => Mnemonic::JMP,
            "lod"  => Mnemonic::LOD,
            "mov"  => Mnemonic::MOV,
            "pop"  => Mnemonic::POP,
            "push" => Mnemonic::PUSH,
            "ret"  => Mnemonic::RET,
            "sto"  => Mnemonic::STO,
            "sub"  => Mnemonic::SUB,
            "tst"  => Mnemonic::TST,
            "xor"  => Mnemonic::XOR,
            s      => SyntaxError::err_from_pair(SyntaxErrorType::MnemonicUnknown(s.to_string()), pair)?,
        };

        Ok(mnemonic)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Register {
    Ra,
    Rb,
    Rc,
    Rd,
    SP,
    PC,
}

impl TryFrom<Pair<'_, Rule>> for Register {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let register = match (pair.as_rule(), pair.as_str().to_ascii_lowercase().as_str()) {
            (Rule::register, "ra") => Register::Ra,
            (Rule::register, "rb") => Register::Rc,
            (Rule::register, "rc") => Register::Rb,
            (Rule::register, "rd") => Register::Rd,
            (Rule::register, "sp") => Register::SP,
            (Rule::register, "pc") => Register::PC,

            _ => SyntaxError::err_from_pair(SyntaxErrorType::RegisterUnknown(pair.as_str().to_string()), pair)?,
        };

        Ok(register)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Statement {
    Directive(Directive),
    Instruction(Instruction),
    EOI,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Operand {
    mode:   Mode,
    target: Target,
}

impl TryFrom<Pair<'_, Rule>> for Operand {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mode = match pair.as_rule() {
            Rule::direct   => Mode::Direct,
            Rule::indirect => Mode::Indirect,

            _ => unreachable!("pest grammar didn't produce a legal addressing mode"),
        };

        let inner : Vec<Pair<'_, Rule>> = pair.into_inner().collect();
        let target = match inner.as_slice() {
            [i]     => Target::try_from(i.clone())?,

            [ ]     => unreachable!("pest grammar didn't produce an operand"),
            [_, ..] => unreachable!("pest grammar produced too many operands"),
        };

        Ok(Self { mode, target })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Target {
    Register(Register),
    Label(Label),
    Word(Word),
}

impl TryFrom<Pair<'_, Rule>> for Target {
    type Error = SyntaxError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value = match pair.as_rule() {
            Rule::register => Self::Register(Register::try_from(pair)?),
            Rule::label    => Self::Label(Label::try_from(pair)?),
            Rule::number   => Self::Word(Word::try_from(pair)?),

            _ => unreachable!("pest grammar produced an unknown target"),
        };

        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mode {
    Direct,
    Indirect
}

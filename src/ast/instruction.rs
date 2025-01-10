use crate::ast::{Label, Word};
use crate::error::{Result, SyntaxError, SyntaxErrorType};
use crate::grammar::{Pair, Rule};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction {
    label:    Option<Label>,
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Operand {
    mode:   Mode,
    target: Target,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mode {
    Direct,
    Indirect
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Target {
    Register(Register),
    Label(Label),
    Word(Word),
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

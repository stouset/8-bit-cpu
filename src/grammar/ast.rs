use ::core::iter::Map;

use crate::grammar::{Cst, CstChildren, Node, NodeRef, Rule};

struct File {
    syntax: NodeRef,
}


enum Statement {
    Directive { syntax: NodeRef },
    Mnemonic  { syntax: NodeRef },
}

enum Numeric {
    U8 { syntax: NodeRef, value: u8 },
    I8 { syntax: NodeRef, value: i8 },
}

enum Directive<'cst> {
    EQU { syntax: NodeRef, label: &'cst str, value: Numeric },
    ORG { value: u8 },
}

enum Mnemonic<'cst> {
    /// No-op.
    NOP,

    /// Halts the computer until it is reset.
    HLT,

    /// Moves data from register `src` to register `dst`.
    MOV { dst: Register, src: Register },

    /// Loads data at the memory address in `src` into register `dst`.
    LOD { dst: Register, src: Address<'cst> },

    /// Stores data from register `src` into the address `dst`.
    STO { dst: Address<'cst>, src: Register },

//         // memory manipulation
//         MOV  { dst:  Register, src:  Register },
//         DATA { dst:  Register, val:  Value },
//         LOD  { dst:  Register, addr: Address },
//         STO  { addr: Address,  src:  Register},

//         // stack
//         PUSH { val: Operand },
//         POP  { reg: Register },

//         // jumps
//         JMP { addr: Operand },
//         JC  { addr: Address },
//         JN  { addr: Address },
//         JO  { addr: Address },
//         JZ  { addr: Address },

//         // functions
//         CALL { reg: Register }, // always RC
//         RET,

//         // ALU
//         NOT { reg: Register }, // only general-purpose registers
//         INC { reg: Register }, // only general-purpose registers
//         DEC { reg: Register }, // only general-purpose registers
//         TST { reg: Register }, // only general-purpose registers

//         // ADD, ADC, SUB, SBC, XOR, OR, AND, CMP
//     }
}

enum Address<'cst> {
    Register(Register),
    Label(Label<'cst>),
}

enum Register {
    RegisterGP(RegisterGP),
    RegisterSP(RegisterSP),
}

enum RegisterGP {
    Ra,
    Rb,
    Rc,
    Rd,
}

enum RegisterSP {
    PC,
    SP,
}

type Label<'cst> = &'cst str;

impl File {
    fn new() -> Self {
        Self { syntax: NodeRef::ROOT }
    }

    fn statements(&self, cst: &Cst<'_>) -> impl Iterator {
        cst.children(self.syntax).filter_map(|syntax| {
            match cst.get(syntax) {
                Node::Rule(Rule::Directive,   _) => Some(Statement::Directive { syntax }),
                Node::Rule(Rule::Mnemonic, _)    => Some(Statement::Mnemonic  { syntax }),
                _                                => None,
            }
        })
    }
}

// impl From<NodeRef> for Statement {
//     fn from(value: NodeRef) -> Self {
//         Self { syntax: value }
//     }
// }

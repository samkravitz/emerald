pub enum Opcode {
    Return = 0,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    Print,
    Pop,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
    GetLocal,
    SetLocal,
    JumpIfFalse,
    Jump,

    Unknown,
}

pub fn from_u8(x: u8) -> Opcode {
    match x {
        0 => Opcode::Return,
        1 => Opcode::Constant,
        2 => Opcode::Negate,
        3 => Opcode::Add,
        4 => Opcode::Subtract,
        5 => Opcode::Multiply,
        6 => Opcode::Divide,
        7 => Opcode::Mod,
        8 => Opcode::Nil,
        9 => Opcode::True,
        10 => Opcode::False,
        11 => Opcode::Not,
        12 => Opcode::Equal,
        13 => Opcode::Greater,
        14 => Opcode::Less,
        15 => Opcode::LogicalAnd,
        16 => Opcode::LogicalOr,
        17 => Opcode::BitwiseAnd,
        18 => Opcode::BitwiseOr,
        19 => Opcode::Print,
        20 => Opcode::Pop,
        21 => Opcode::DefineGlobal,
        22 => Opcode::GetGlobal,
        23 => Opcode::SetGlobal,
        24 => Opcode::GetLocal,
        25 => Opcode::SetLocal,
        26 => Opcode::JumpIfFalse,
        27 => Opcode::Jump,
        _ => Opcode::Unknown,
    }
}

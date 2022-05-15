pub enum Instruction {
    ADD(String),
}

pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}
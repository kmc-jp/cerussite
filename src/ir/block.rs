use std::vec::Vec;
use super::instruction::Instruction;
use super::register::Register;

struct BasicBlock(Register, Vec<Instruction>);
impl BasicBlock {
    fn new(reg: Register) -> BasicBlock {
        let vec = Vec::new();
        BasicBlock(reg, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_block() {
        let reg = Register::new();
        let _bb = BasicBlock::new(reg);
    }
}

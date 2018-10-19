use std::vec::Vec;
use super::instruction::Instruction;
use super::register::Register;

struct BasicBlock(Register, Vec<Instruction>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_block() {
        let reg = Register::new();
        let vec = Vec::new();
        let _bb = BasicBlock(reg, vec);
    }
}

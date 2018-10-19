use std::vec::Vec;
use super::instruction::Instruction;
use super::register::Register;

struct BasicBlock(Register, Vec<Instruction>);
impl BasicBlock {
    fn new(reg: Register) -> BasicBlock {
        let vec = Vec::new();
        BasicBlock(reg, vec)
    }
    fn push(&mut self, inst: Instruction) {
        self.1.push(inst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::value::*;

    #[test]
    fn test_basic_block() {
        let reg = Register::new();
        let mut bb = BasicBlock::new(reg);
        let val = Value::Constant(0);
        let inst = Instruction::Ret(val);
        bb.push(inst);
    }
}

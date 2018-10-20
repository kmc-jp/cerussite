use std::vec::Vec;
use super::instruction::Instruction;
use super::register::Register;
use super::value::Value;

pub struct BasicBlock(Register, Vec<Instruction>);
impl BasicBlock {
    pub fn new() -> BasicBlock {
        let reg = Register::new();
        let vec = Vec::new();
        BasicBlock(reg, vec)
    }
    pub fn push(&mut self, inst: Instruction) {
        self.1.push(inst)
    }
    pub fn label(&self) -> Value {
        let reg = self.0.clone();
        Value::Label(reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_block() {
        let mut bb = BasicBlock::new();
        let val = Value::Constant(0);
        let inst = Instruction::Ret(val);
        let _label = bb.label();
        bb.push(inst);
    }
}

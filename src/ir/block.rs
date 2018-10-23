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
    fn push(&mut self, inst: Instruction) {
        self.1.push(inst)
    }
    pub fn label(&self) -> Value {
        let reg = self.0.clone();
        Value::Label(reg)
    }
    pub fn ret(&mut self, val: Value) {
        let ret = Instruction::Ret(val);
        self.push(ret)
    }
    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let reg = Register::new();
        let add = Instruction::Add(reg.clone(), lhs, rhs);
        self.push(add);
        Value::Register(reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_block() {
        let mut bb = BasicBlock::new();
        let lhs = Value::Constant(0);
        let rhs = Value::Constant(1);
        let add = bb.add(lhs, rhs);
        bb.ret(add);
        let _label = bb.label();
    }
}

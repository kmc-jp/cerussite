use std::vec::Vec;
use super::block::BasicBlock;
use super::instruction::Instruction;
use super::register::Register;
use super::value::Value;

pub struct Builder(Vec<BasicBlock>);
impl Builder {
    pub fn new() -> Builder {
        let vec = Vec::new();
        Builder(vec)
    }
    fn push(&mut self, inst: Instruction) {
        let last = self.0.len() - 1;
        self.0[last].push(inst)
    }
    pub fn block(&mut self) -> Value {
        let reg = Register::new();
        let bb = BasicBlock::new(reg.clone());
        self.0.push(bb);
        Value::Register(reg)
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

    fn test_builder() {
        let mut builder = Builder::new();
        let _bl = builder.block();
        let lhs = Value::Constant(0);
        let rhs = Value::Constant(1);
        let add = builder.add(lhs, rhs);
        builder.ret(add);
    }
}

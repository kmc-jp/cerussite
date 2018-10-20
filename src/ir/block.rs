use std::fmt;
use std::vec::Vec;
use super::instruction::Instruction;
use super::value::Register;
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
    pub fn print_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "; <label>:{}:", self.0)
    }
    pub fn numbering(&self, init: i32) -> i32 {
        let mut init = self.0.set(init);
        for inst in &self.1 {
            init = inst.numbering(init);
        }
        init
    }
    pub fn label(&self) -> Value {
        let weak = self.0.make_ref();
        Value::Label(weak)
    }
    pub fn ret(&mut self, val: Value) {
        let ret = Instruction::Ret(val);
        self.push(ret)
    }
    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let reg = Register::new();
        let weak = reg.make_ref();
        let add = Instruction::Add(reg, lhs, rhs);
        self.push(add);
        Value::Register(weak)
    }
}
impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for inst in &self.1 {
            writeln!(f, "  {}", inst);
        }
        Ok(())
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
        let _end = bb.numbering(0);
        println!("{}", bb);
    }
}

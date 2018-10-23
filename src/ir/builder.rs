use std::vec::Vec;
use super::instruction::Instruction;
use super::register::Register;
use super::value::Value;

pub struct Builder(Vec<Instruction>);
impl Builder {
    pub fn new() -> Builder {
        let vec = Vec::new();
        Builder(vec)
    }
    fn push(&mut self, inst: Instruction) {
        self.0.push(inst)
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
        let lhs = Value::Constant(0);
        let rhs = Value::Constant(1);
        let add = builder.add(lhs, rhs);
        builder.ret(add);
    }
}

use super::instruction::Instruction;
use super::register::Register;
use super::value::Value;

pub struct Builder();
impl Builder {
    pub fn new() -> Builder {
        Builder()
    }
    pub fn ret(&self, val: Value) -> Instruction {
        Instruction::Ret(val)
    }
    pub fn add(&self, lhs: Value, rhs: Value) -> Instruction {
        let reg = Register::new();
        Instruction::Add(reg, lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_builder() {
        let builder = Builder::new();
        let lhs = Value::Constant(0);
        let rhs = Value::Constant(1);
        let add = builder.add(lhs, rhs);
        let target = add.target().unwrap();
        let _ret = builder.ret(target);
    }
}

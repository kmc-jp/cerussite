use super::instruction::Instruction;
use super::register::IdentityGenerator;
use super::register::Register;
use super::value::Value;

pub struct Builder(IdentityGenerator);
impl Builder {
    pub fn new() -> Builder {
        let gen = IdentityGenerator::new();
        Builder(gen)
    }
    pub fn ret(&self, val: Value) -> Instruction {
        Instruction::Ret(val)
    }
    pub fn add(&self, lhs: Value, rhs: Value) -> Instruction {
        let reg = Register::new();
        Instruction::Add(reg, lhs, rhs)
    }
}
#[test]
fn test_builder() {
    let builder = Builder::new();
    let lhs = Value::Constant(0);
    let rhs = Value::Constant(1);
    let add = builder.add(lhs, rhs);
    let target = add.target().unwrap();
    let _ret = builder.ret(target);
}

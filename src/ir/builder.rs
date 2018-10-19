use super::instruction::Instruction;
use super::register::IdentityGenerator;
use super::register::Reg;
use super::value::Value;

pub struct Builder(IdentityGenerator);
impl<'a> Builder {
    pub fn new() -> Builder {
        let gen = IdentityGenerator::new();
        Builder(gen)
    }
    pub fn ret(&self, val: Value<'a>) -> Instruction<'a> {
        Instruction::Ret(val)
    }
    pub fn add(&self, lhs: Value<'a>, rhs: Value<'a>) -> Instruction<'a> {
        let reg = Reg::new(&self.0);
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

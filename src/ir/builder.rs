use super::instruction::Instruction;
use super::instruction::Value;
use super::register::IdentityGenerator;
use super::register::Register;

struct Builder(IdentityGenerator);
impl<'a> Builder {
    fn new() -> Builder {
        let gen = IdentityGenerator::new();
        Builder(gen)
    }
    fn ret(&self, val: Value<'a>) -> Instruction<'a> {
        Instruction::Ret(val)
    }
    fn add(&self, lhs: Value<'a>, rhs: Value<'a>) -> Instruction<'a> {
        let reg = Register::new(&self.0);
        Instruction::Add(reg, lhs, rhs)
    }
}
#[test]
fn test_builder() {
    let builder = Builder::new();
    let lhs = Value::Constant(0);
    let rhs = Value::Constant(1);
    let add = builder.add(lhs, rhs);
    let target = Instruction::target(&add).unwrap();
    let _ret = builder.ret(target);
}

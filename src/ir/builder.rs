use super::instruction::Instruction;
use super::instruction::Value;
use super::register::IdentityGenerator;

struct Builder(IdentityGenerator);
impl<'a> Builder {
    fn new() -> Builder {
        let gen = IdentityGenerator::new();
        Builder(gen)
    }
    fn ret(&self, val: Value<'a>) -> Instruction<'a> {
        Instruction::Ret(val)
    }
}
#[test]
fn test_builder() {
    let builder = Builder::new();
    let val = Value::Constant(0);
    let _ret = builder.ret(val);
}

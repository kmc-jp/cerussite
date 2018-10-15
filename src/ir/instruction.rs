use super::register::IdentityGenerator;
use super::register::Register;

enum Value<'a> {
    Constant(i32),
    Register(&'a Register),
}
#[test]
fn test_value() {
    let a = IdentityGenerator::new();
    let b = Register::new(&a);
    let _c = Value::Constant(0);
    let _d = Value::Register(&b);
}

enum Instruction<'a> {
    Ret(Value<'a>),
    Add(Register, Value<'a>, Value<'a>),
}
impl<'a> Instruction<'a> {
    fn ret(val: Value) -> Instruction {
        Instruction::Ret(val)
    }
    fn add(gen: &'a IdentityGenerator,
           lhs: Value<'a>, rhs: Value<'a>) -> Instruction<'a> {
        let reg = Register::new(gen);
        Instruction::Add(reg, lhs, rhs)
    }
    fn target(inst: &'a Instruction) -> Option<Value<'a>> {
        match inst {
            Instruction::Ret(_) => None,
            Instruction::Add(target, _, _) => Some(Value::Register(&target)),
        }
    }
}
#[test]
fn test_instruction() {
    let gen = IdentityGenerator::new();
    let val1 = Value::Constant(1);
    let val2 = Value::Constant(2);
    let add = Instruction::add(&gen, val1, val2);
    let val3 = Instruction::target(&add).unwrap();
    let _ret = Instruction::ret(val3);
}

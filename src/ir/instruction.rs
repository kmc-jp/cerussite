use super::register::IdentityGenerator;
use super::register::Register;

enum Value<'a> {
    Constant(i32),
    Register(&'a Register),
}
#[test]
fn test_value() {
    let mut a = IdentityGenerator::new();
    let b = Register::new(&mut a);
    let _c = Value::Constant(0);
    let _d = Value::Register(&b);
}

enum Instruction<'a> {
    Ret(Value<'a>),
    Add(Register, Value<'a>, Value<'a>),
}
#[test]
fn test_instruction() {
    let mut gen = IdentityGenerator::new();
    let reg1 = Register::new(&mut gen);
    let reg2 = Register::new(&mut gen);
    let reg3 = Register::new(&mut gen);
    let val1 = Value::Register(&reg1);
    let val2 = Value::Register(&reg2);
    let val3 = Value::Constant(0);
    let _ret = Instruction::Ret(val1);
    let _add = Instruction::Add(reg3, val2, val3);
}

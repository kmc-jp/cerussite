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
fn make_ret_instruction(val: Value) -> Instruction {
    Instruction::Ret(val)
}
fn make_add_instruction<'a>(gen: &'a mut IdentityGenerator,
                            lhs: Value<'a>, rhs: Value<'a>) -> Instruction<'a> {
    let reg = Register::new(gen);
    Instruction::Add(reg, lhs, rhs)
}
#[test]
fn test_instruction() {
    let mut gen = IdentityGenerator::new();
    let val1 = Value::Constant(1);
    let val2 = Value::Constant(2);
    let val3 = Value::Constant(3);
    let _add = make_add_instruction(&mut gen, val1, val2);
    let _ret = make_ret_instruction(val3);
}

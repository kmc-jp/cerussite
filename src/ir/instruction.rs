use super::register::IdentityGenerator;
use super::register::Register;

enum Value {
    Constant(i32),
    Register(Register),
}
#[test]
fn test_value() {
    let mut a = IdentityGenerator::new();
    let b = Register::new(&mut a);
    let _c = Value::Constant(0);
    let _d = Value::Register(b);
}

enum Instruction {
    Ret(Value),
}
#[test]
fn test_instruction() {
    let a = Value::Constant(0);
    let _b = Instruction::Ret(a);
}

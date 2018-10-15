use super::register::IdentityGenerator;
use super::register::Register;

pub enum Value<'a> {
    Constant(i32),
    Register(&'a Register),
}

pub enum Instruction<'a> {
    Ret(Value<'a>),
    Add(Register, Value<'a>, Value<'a>),
}
impl<'a> Instruction<'a> {
    pub fn target(&'a self) -> Option<Value<'a>> {
        match self {
            Instruction::Ret(_) => None,
            Instruction::Add(target, _, _) => Some(Value::Register(&target)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let a = IdentityGenerator::new();
        let b = Register::new(&a);
        let _c = Value::Constant(0);
        let _d = Value::Register(&b);
    }

    #[test]
    fn test_instruction() {
        let gen = IdentityGenerator::new();
        let val1 = Value::Constant(1);
        let val2 = Value::Constant(2);
        let reg = Register::new(&gen);
        let add = Instruction::Add(reg, val1, val2);
        let val3 = add.target().unwrap();
        let _ret = Instruction::Ret(val3);
    }
}

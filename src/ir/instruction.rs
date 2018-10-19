use super::register::Reg;
use super::value::Value;

pub enum Instruction<'a> {
    Ret(Value<'a>),
    Add(Reg, Value<'a>, Value<'a>),
}
impl<'a> Instruction<'a> {
    pub fn target(&'a self) -> Option<Value<'a>> {
        match self {
            Instruction::Ret(_) => None,
            Instruction::Add(target, _, _) => Some(Value::Reg(&target)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::register::*;

    #[test]
    fn test_instruction() {
        let gen = IdentityGenerator::new();
        let val1 = Value::Constant(1);
        let val2 = Value::Constant(2);
        let reg = Reg::new(&gen);
        let add = Instruction::Add(reg, val1, val2);
        let val3 = add.target().unwrap();
        let _ret = Instruction::Ret(val3);
    }
}

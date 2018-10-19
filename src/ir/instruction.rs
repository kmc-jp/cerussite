use super::register::Register;
use super::value::Value;

pub enum Instruction {
    Ret(Value),
    Add(Register, Value, Value),
}
impl Instruction {
    pub fn target(&self) -> Option<Value> {
        match self {
            Instruction::Ret(_) => None,
            Instruction::Add(target, _, _) => Some(Value::Register(target.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        let val1 = Value::Constant(1);
        let val2 = Value::Constant(2);
        let reg = Register::new();
        let add = Instruction::Add(reg, val1, val2);
        let val3 = add.target().unwrap();
        Instruction::Ret(val3);
    }
}

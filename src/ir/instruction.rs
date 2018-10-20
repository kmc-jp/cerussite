use super::value::Register;
use super::value::Value;

pub enum Instruction {
    Ret(Value),
    Add(Register, Value, Value),
}
impl Instruction {
    pub fn numbering(&self, init: i32) -> i32 {
        match self {
            Instruction::Ret(_) => init,
            Instruction::Add(reg, _, _) => reg.set(init),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        let reg = Register::new();
        let reg1 = Register::new();
        let reg2 = Register::new();
        let reg3 = Register::new();
        let val1 = Value::Register(reg1.make_ref());
        let val2 = Value::Register(reg2.make_ref());
        let val3 = Value::Register(reg3.make_ref());
        let add = Instruction::Add(reg, val1, val2);
        let ret = Instruction::Ret(val3);
        let mut init = 0;
        init = add.numbering(init);
        let _end = ret.numbering(init);
    }
}

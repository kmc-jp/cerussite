use std::fmt;
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
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Ret(val) =>
                write!(f, "ret i32 {}", val),
            Instruction::Add(reg, lhs, rhs) => {
                let target = Value::Register(reg.make_ref());
                write!(f, "{} = add i32 {}, {}", target, lhs, rhs)
            },
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
        init = reg1.set(init);
        init = reg2.set(init);
        init = reg3.set(init);
        init = add.numbering(init);
        let _end = ret.numbering(init);
        assert_eq!(add.to_string(), "%3 = add i32 %0, %1");
        assert_eq!(ret.to_string(), "ret i32 %2");
    }
}

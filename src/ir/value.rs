use super::register::Reg;
use super::register::Register;

pub enum Value<'a> {
    Constant(i32),
    Register(Register),
    Reg(&'a Reg),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let a = Register::new();
        let _b = Value::Constant(0);
        let _c = Value::Register(a);
    }
}

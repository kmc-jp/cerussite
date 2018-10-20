use super::register::Register;

pub enum Value {
    Constant(i32),
    Register(Register),
    Label(Register),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let a = Register::new();
        let _b = Value::Constant(0);
        let _c = Value::Register(a.clone());
        let _c = Value::Label(a.clone());
    }
}

use super::register::Register;

pub enum Value<'a> {
    Constant(i32),
    Register(&'a Register),
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::register::*;

    #[test]
    fn test_value() {
        let a = IdentityGenerator::new();
        let b = Register::new(&a);
        let _c = Value::Constant(0);
        let _d = Value::Register(&b);
    }
}

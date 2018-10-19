use super::register::Reg;

pub enum Value<'a> {
    Constant(i32),
    Reg(&'a Reg),
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::register::*;

    #[test]
    fn test_value() {
        let a = IdentityGenerator::new();
        let b = Reg::new(&a);
        let _c = Value::Constant(0);
        let _d = Value::Reg(&b);
    }
}

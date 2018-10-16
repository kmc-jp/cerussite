#![allow(dead_code)]

mod builder;
mod instruction;
mod register;

#[cfg(test)]
mod tests {
    use super::builder::*;
    use super::instruction::*;

    #[test]
    fn test() {
        let b = Builder::new();
        let v0 = Value::Constant(0);
        let v1 = Value::Constant(1);
        let v2 = Value::Constant(2);
        let v3 = Value::Constant(3);
        let a0 = b.add(v0, v1);
        let a1 = b.add(v2, v3);
        let a2 = b.add(a0.target().unwrap(), a1.target().unwrap());
        let _ = b.ret(a2.target().unwrap());
    }
}

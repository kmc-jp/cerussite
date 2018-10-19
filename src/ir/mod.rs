#![allow(dead_code)]

mod builder;
mod instruction;
mod register;
mod value;

#[cfg(test)]
mod tests {
    use super::builder::*;
    use super::value::*;

    #[test]
    fn test() {
        let mut b = Builder::new();
        let v0 = Value::Constant(0);
        let v1 = Value::Constant(1);
        let v2 = Value::Constant(2);
        let v3 = Value::Constant(3);
        let a0 = b.add(v0, v1);
        let a1 = b.add(v2, v3);
        let a2 = b.add(a0, a1);
        b.ret(a2);
    }
}

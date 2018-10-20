use std::cell::Cell;
use std::rc::Rc;
use std::rc::Weak;

enum RegisterName {
    Unnamed(),
    Numbering(i32),
}

pub struct Register(Rc<Cell<RegisterName>>);
impl Register {
    pub fn new() -> Register {
        let name = RegisterName::Unnamed();
        Register(Rc::new(Cell::new(name)))
    }
    pub fn set(&self, n: i32) -> i32 {
        let name = RegisterName::Numbering(n);
        self.0.set(name);
        n + 1
    }
    pub fn make_ref(&self) -> WeakRegister {
        let name = Rc::downgrade(&self.0);
        WeakRegister(name)
    }
}

pub struct WeakRegister(Weak<Cell<RegisterName>>);

pub enum Value {
    Constant(i32),
    Register(WeakRegister),
    Label(WeakRegister),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_name() {
        let _a = RegisterName::Unnamed();
        let _b = RegisterName::Numbering(0);
    }

    #[test]
    fn test_register() {
        let a = Register::new();
        let _b = a.make_ref();
        let _c = a.set(0);
    }

    #[test]
    fn test_weak_register() {
        let a = Weak::new();
        let _b = WeakRegister(a);
    }

    #[test]
    fn test_value() {
        let a = Register::new();
        let _b = Value::Constant(0);
        let _c = Value::Register(a.make_ref());
        let _c = Value::Label(a.make_ref());
    }
}

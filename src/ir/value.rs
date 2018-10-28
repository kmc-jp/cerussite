use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Copy, Clone)]
enum RegisterName {
    Unnamed(),
    Numbering(i32),
}
impl fmt::Display for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterName::Unnamed() => panic!(),
            RegisterName::Numbering(n) => write!(f, "{}", n),
        }
    }
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
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.0.get();
        write!(f, "{}", name)
    }
}

pub struct WeakRegister(Weak<Cell<RegisterName>>);
impl fmt::Display for WeakRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.0.upgrade().unwrap().get();
        write!(f, "{}", name)
    }
}

pub enum Value {
    Constant(i32),
    Register(WeakRegister),
    Label(WeakRegister),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Constant(n) => write!(f, "{}", n),
            Value::Register(weak) => write!(f, "%{}", weak),
            Value::Label(weak) => write!(f, "%{}", weak),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_name() {
        let _a = RegisterName::Unnamed();
        let b = RegisterName::Numbering(0);
        assert_eq!(b.to_string(), "0");
    }

    #[test]
    fn test_register() {
        let a = Register::new();
        let _b = a.make_ref();
        let _c = a.set(0);
        assert_eq!(a.to_string(), "0");
    }

    #[test]
    fn test_weak_register() {
        let a = Weak::new();
        let _b = WeakRegister(a);
        let c = Register::new();
        let d = c.make_ref();
        c.set(0);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_value() {
        let a = Register::new();
        let b = Value::Constant(0);
        let c = Value::Register(a.make_ref());
        let d = Value::Label(a.make_ref());
        a.set(0);
        assert_eq!(b.to_string(), "0");
        assert_eq!(c.to_string(), "%0");
        assert_eq!(d.to_string(), "%0");
    }
}

use std::cell::Cell;
use std::rc::Rc;

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
    pub fn set(&self, n: i32) {
        let name = RegisterName::Numbering(n);
        self.0.set(name)
    }
}
impl Clone for Register {
    fn clone(&self) -> Register {
        Register(self.0.clone())
    }
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
        let b = a.clone();
        b.set(0);
    }
}

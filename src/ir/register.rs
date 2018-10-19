use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct Identity(Cell<i32>);
impl Identity {
    fn new() -> Identity {
        Identity(Cell::new(0))
    }
    fn next(&self) -> Identity {
        let prev = self.0.get();
        self.0.set(prev + 1);
        Identity(Cell::new(prev))
    }
}

pub struct IdentityGenerator(Identity);
impl IdentityGenerator {
    pub fn new() -> IdentityGenerator {
        let id = Identity::new();
        IdentityGenerator(id)
    }
    fn generate(&self) -> Identity {
        self.0.next()
    }
}

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
    fn test_identity() {
        let a = Identity::new();
        let b = Identity::new();
        let c = b.next();
        assert_ne!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn test_identity_generator() {
        let a = IdentityGenerator::new();
        let b = a.generate();
        let c = a.generate();
        assert_ne!(b, c);
    }

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

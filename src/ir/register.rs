use std::cell::Cell;

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
#[test]
fn test_identity() {
    let a = Identity::new();
    let b = Identity::new();
    let c = b.next();
    assert_ne!(a, b);
    assert_eq!(a, c);
}

pub struct IdentityGenerator(Identity);
impl IdentityGenerator {
    pub fn new() -> IdentityGenerator {
        let id = Identity::new();
        IdentityGenerator(id)
    }
    fn generate(&mut self) -> Identity {
        self.0.next()
    }
}
#[test]
fn test_identity_generator() {
    let mut a = IdentityGenerator::new();
    let b = a.generate();
    let c = a.generate();
    assert_ne!(b, c);
}

#[derive(Debug)]
pub struct Register(Identity);
impl PartialEq for Register {
    fn eq(&self, other: &Register) -> bool {
        self.0 == other.0
    }
}
impl Eq for Register {}
impl Register {
    pub fn new(gen: &mut IdentityGenerator) -> Register {
        Register(gen.generate())
    }
}
#[test]
fn test_register() {
    let mut a = IdentityGenerator::new();
    let b = Register::new(&mut a);
    let c = Register::new(&mut a);
    assert_ne!(b, c);
}

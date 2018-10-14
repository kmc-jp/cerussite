#[derive(Debug, Eq, PartialEq)]
struct Identity(i32);
impl Identity {
    fn next(&mut self) -> Identity {
        let prev = self.0;
        self.0 += 1;
        Identity(prev)
    }
}
#[test]
fn test_identity() {
    let a = Identity(0);
    let mut b = Identity(0);
    let c = b.next();
    assert!(a != b);
    assert!(a == c);
}

struct IdentityGenerator(Identity);
impl IdentityGenerator {
    fn new() -> IdentityGenerator {
        let id = Identity(0);
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
    assert!(b != c);
}

#[derive(Debug)]
struct Register(Identity);
impl PartialEq for Register {
    fn eq(&self, other: &Register) -> bool {
        self.0 == other.0
    }
}
impl Eq for Register {}
impl Register {
    fn new(gen: &mut IdentityGenerator) -> Register {
        Register(gen.generate())
    }
}
#[test]
fn test_register() {
    let mut a = IdentityGenerator::new();
    let b = Register::new(&mut a);
    let c = Register::new(&mut a);
    assert!(b != c);
}

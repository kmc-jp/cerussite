struct Identity(i32);
impl Clone for Identity {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Identity {}
impl PartialEq for Identity {
    fn eq(&self, other: &Identity) -> bool {
        self.0 == other.0
    }
}
impl Eq for Identity {}
impl Identity {
    fn next(&mut self) {
        self.0 += 1;
    }
}
#[test]
fn test_identity() {
    let a = Identity(0);
    let b = a;
    let mut c = b;
    c.next();
    assert!(a == b);
    assert!(a != c);
}

struct IdentityGenerator(Identity);
impl IdentityGenerator {
    fn new() -> IdentityGenerator {
        let id = Identity(0);
        IdentityGenerator(id)
    }
    fn generate(&mut self) -> Identity {
        let prev = self.0;
        self.0.next();
        prev
    }
}
#[test]
fn test_identity_generator() {
    let mut a = IdentityGenerator::new();
    let b = a.generate();
    let c = a.generate();
    assert!(b != c);
}

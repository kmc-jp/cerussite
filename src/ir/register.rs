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
#[test]
fn test_identity() {
    let a = Identity(0);
    let b = a;
    let c = Identity(1);
    assert!(a == b);
    assert!(a != c);
}

struct IdentityGenerator(Identity);
#[test]
fn test_identity_generator() {
    let a = Identity(0);
    let _ = IdentityGenerator(a);
}

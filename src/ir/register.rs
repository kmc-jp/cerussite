struct Identity(i32);
impl Clone for Identity {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Identity {}
#[test]
fn test_identity() {
    let _a = Identity(0);
    let _b = _a;
}

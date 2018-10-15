use super::register::IdentityGenerator;

struct Builder(IdentityGenerator);
impl Builder {
    fn new() -> Builder {
        let gen = IdentityGenerator::new();
        Builder(gen)
    }
}
#[test]
fn test_builder() {
    let _builder = Builder::new();
}

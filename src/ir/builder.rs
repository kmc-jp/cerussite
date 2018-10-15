use super::register::IdentityGenerator;

struct Builder(IdentityGenerator);
#[test]
fn test_builder() {
    let gen = IdentityGenerator::new();
    let _builder = Builder(gen);
}

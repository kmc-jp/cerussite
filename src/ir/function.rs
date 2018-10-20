use std::vec::Vec;
use super::block::BasicBlock;

struct Function(String, Vec<BasicBlock>);
impl Function {
    fn new() -> Function {
        let name = String::from("main");
        let vec = Vec::new();
        Function(name, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let _func = Function::new();
    }
}

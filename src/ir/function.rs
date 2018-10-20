use std::vec::Vec;
use super::block::BasicBlock;

struct Function(String, Vec<BasicBlock>);
impl Function {
    fn new() -> Function {
        let name = String::from("main");
        let vec = Vec::new();
        Function(name, vec)
    }
    fn push(&mut self, block: BasicBlock) {
        self.1.push(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::register::Register;

    #[test]
    fn test_function() {
        let mut func = Function::new();
        let reg = Register::new();
        let block = BasicBlock::new(reg);
        func.push(block);
    }
}

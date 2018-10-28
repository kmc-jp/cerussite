use std::fmt;
use std::vec::Vec;
use super::block::BasicBlock;

pub struct Function(String, Vec<BasicBlock>);
impl Function {
    pub fn new() -> Function {
        let name = String::from("main");
        let vec = Vec::new();
        Function(name, vec)
    }
    pub fn push(&mut self, block: BasicBlock) {
        self.1.push(block)
    }
    fn numbering(&self) {
        let mut init = 0;
        for block in &self.1 {
            init = block.numbering(init);
        }
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.numbering();
        writeln!(f, "define i32 @{}() {{", self.0)?;
        let mut first = true;
        for block in &self.1 {
            if !first {
                writeln!(f, "")?;
                block.print_header(f)?;
            }
            write!(f, "{}", block)?;
            first = false;
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let mut func = Function::new();
        let block = BasicBlock::new();
        func.push(block);
        func.numbering();
        let expect = "define i32 @main() {\n}\n";
        assert_eq!(func.to_string(), expect);
    }
}

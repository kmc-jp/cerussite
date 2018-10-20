use std::vec::Vec;
use super::block::BasicBlock;
use super::instruction::Instruction;
use super::value::Value;

pub struct Builder(Vec<BasicBlock>);
impl Builder {
    pub fn new() -> Builder {
        let vec = Vec::new();
        Builder(vec)
    }
    fn push(&mut self, inst: Instruction) {
        let last = self.0.len() - 1;
        self.0[last].push(inst)
    }
    pub fn block(&mut self) -> Value {
        let bb = BasicBlock::new();
        let label = bb.label();
        self.0.push(bb);
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_builder() {
        let mut builder = Builder::new();
        let _bl = builder.block();
    }
}

use std::vec::Vec;
use super::block::BasicBlock;

struct Function(String, Vec<BasicBlock>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let str = String::from("main");
        let vec = Vec::new();
        let _func = Function(str, vec);
    }
}

use std::collections::HashMap;

/// temporary struct: this should be unnecessary when LLVM IR module fully matures.
pub struct CodeGenState {
    reg: usize,
    pub vars: HashMap<String, usize>,
}

impl CodeGenState {
    pub fn new() -> CodeGenState {
        CodeGenState {
            reg: 0,
            vars: HashMap::new(),
        }
    }

    pub fn next_reg(&mut self) -> usize {
        self.reg += 1;
        self.reg
    }
}

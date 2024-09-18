use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Definition {
    Variable,
    Module,
    Function
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {}

impl FunctionDefinition {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_ref() -> ModuleDefinitionRef {
        Rc::new(Self::new())
    }
}

pub type ModuleDefinitionRef = Rc<FunctionDefinition>;
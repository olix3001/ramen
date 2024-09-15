use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Scope {}

pub type ScopeRef = Rc<Scope>;
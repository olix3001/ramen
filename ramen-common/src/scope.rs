use std::{cell::RefCell, rc::Rc};

use hashbrown::HashMap;

use crate::ast::NodeId;

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Option<ScopeRef>
}

pub type ScopeRef = Rc<Scope>;

impl Scope {
    pub fn new(parent: Option<ScopeRef>) -> Self {
        Self {
            parent
        }
    }

    pub fn new_ref(parent: Option<ScopeRef>) -> ScopeRef {
        Rc::new(Self::new(parent))
    }
}

#[derive(Debug, Clone)]
pub struct ScopeMapRef {
    scopes: RefCell<HashMap<NodeId, ScopeRef>>,
}

impl ScopeMapRef {
    pub fn new() -> Self {
        let self_ = Self {
            scopes: RefCell::default(),
        };
        self_
    }

    pub fn get_or_new(&self, id: NodeId, parent: Option<ScopeRef>) -> ScopeRef {
        let scope = self.scopes.borrow().get(&id).cloned();
        match scope {
            Some(scope) => scope,
            None => self.add(id, parent)
        }
    }

    pub fn add(&self, id: NodeId, parent: Option<ScopeRef>) -> ScopeRef {
        let scope = Scope::new_ref(parent);
        self.scopes.borrow_mut().insert(id, scope.clone());
        scope
    }

    pub fn get(&self, id: NodeId) -> Option<ScopeRef> {
        self.scopes.borrow().get(&id).cloned()
    }
}
use std::{cell::RefCell, rc::Rc};

use hashbrown::HashMap;

use crate::ast::NodeId;

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Option<ScopeRef>,
    pub namespaces: [RefCell<HashMap<String, NodeId>>; 2],
}

pub type ScopeRef = Rc<Scope>;

impl Scope {
    pub const NS_NAMES: usize = 0;
    pub const NS_TYPES: usize = 1;

    pub fn new(parent: Option<ScopeRef>) -> Self {
        Self {
            parent,
            namespaces: [
                RefCell::default(),
                RefCell::default(),
            ]
        }
    }

    pub fn new_ref(parent: Option<ScopeRef>) -> ScopeRef {
        Rc::new(Self::new(parent))
    }

    fn define(
        &self,
        namespace: usize,
        name: impl AsRef<str>,
        id: NodeId
    ) {
        let mut ns = self.namespaces[namespace].borrow_mut();
        ns.insert(name.as_ref().to_string(), id);
    }

    fn search<F, U>(
        &self,
        namespace: usize,
        name: impl AsRef<str>,
        transform: F,
        default: U
    ) -> U
    where F: Fn(NodeId) -> U {
        if let Some(symbol) = self.namespaces[namespace].borrow().get(name.as_ref()) {
            transform(*symbol)
        } else if let Some(parent) = &self.parent {
            parent.search(namespace, name, transform, default)
        } else { 
            default
        }
    }

    // ==< Names >==
    pub fn define_name(&self, name: impl AsRef<str>, id: NodeId) {
        self.define(Scope::NS_NAMES, name, id);
    }
    pub fn search_name(&self, name: impl AsRef<str>) -> Option<NodeId> {
        self.search(Scope::NS_NAMES, name, Some, None)
    }

    // ==< Types >==
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
        let previous = self.scopes.borrow_mut().insert(id, scope.clone());
        #[cfg(debug_assertions)]
        if previous.is_some() {
            panic!("Attempt to redefine scope for {id}");
        }
        scope
    }

    pub fn get(&self, id: NodeId) -> Option<ScopeRef> {
        self.scopes.borrow().get(&id).cloned()
    }
}
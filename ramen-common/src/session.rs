use std::cell::{Cell, RefCell};

use hashbrown::HashMap;
use slotmap::SlotMap;

use crate::{ast::NodeId, defs::Definition, error::Diagnostic, scope::ScopeMapRef, types::RamenType};

slotmap::new_key_type! {
    pub struct SourceId;
}

impl SourceId {
    pub fn dummy() -> Self {
        Self::from(slotmap::KeyData::from_ffi(0))
    }
}

/// Compiler session containing cache, name bindings and more.
#[derive(Debug, Clone)]
pub struct Session {
    pub sources: RefCell<SlotMap<SourceId, RamenSource>>,
    pub errors: Cell<usize>,

    pub scopes: ScopeMapRef,
    pub refs: RefCell<HashMap<NodeId, NodeId>>,
    pub defs: RefCell<HashMap<NodeId, Definition>>,
    pub types: RefCell<HashMap<NodeId, RamenType>>,
    pub symbols: RefCell<HashMap<NodeId, String>>
}

impl Session {
    pub fn new() -> Self {
        Self {
            sources: RefCell::default(),
            errors: Cell::new(0),

            scopes: ScopeMapRef::new(),
            refs: RefCell::default(),
            defs: RefCell::default(),
            types: RefCell::default(),
            symbols: RefCell::default(),
        }
    }

    // ==< Ref-related >==
    pub fn set_ref(&self, source: NodeId, target: NodeId) {
        self.refs.borrow_mut().insert(source, target);
    }

    pub fn get_ref_target(&self, id: NodeId) -> Option<NodeId> {
        self.refs.borrow().get(&id).cloned()
    }

    // ==< Def-related >==
    pub fn alloc_def(&self, ref_id: NodeId) -> NodeId {
        let def_id = NodeId::next();
        self.set_ref(ref_id, def_id);
        def_id
    }

    pub fn set_def(&self, def_id: NodeId, def: Definition) {
        #[cfg(debug_assertions)]
        if let Some(existing) = self.defs.borrow().get(&def_id) {
            panic!("Trying to override definition {def_id} with {def:?}, but is is already set to {existing:?}");
        }

        self.defs.borrow_mut().insert(def_id, def);
    }

    pub fn get_def(&self, def_id: NodeId) -> Option<Definition> {
        self.defs.borrow().get(&def_id).cloned()
    }

    // ==< Type-related >==
    pub fn try_bind_type(&self, target: NodeId, source: NodeId) -> bool {
        if let Some(ty) = self.get_type(source) {
            self.set_type(target, ty);
            true
        } else { false }
    }
    pub fn set_type(&self, id: NodeId, ty: RamenType) {
        self.types.borrow_mut().insert(id, ty);
    }
    pub fn get_type(&self, id: NodeId) -> Option<RamenType> {
        self.types.borrow_mut().get(&id).cloned()
    }

    // ==< Symbol-related >==
    pub fn set_symbol(&self, node: NodeId, symbol: impl AsRef<str>) {
        self.symbols.borrow_mut().insert(node, symbol.as_ref().to_string());
    }

    pub fn get_symbol(&self, node: NodeId) -> Option<String> {
        self.symbols.borrow().get(&node).cloned()
    }

    // ==< Reporting >==
    pub fn print_diagnostic(&self, diag: &dyn Diagnostic) {
        if diag.is_fatal() {
            self.errors.set(self.errors.get() + 1);
        }

        let report = diag.build_report(&self);

        eprintln!("{report:?}") // TODO: Replace with proper error printing after implementing source cache
    }

    pub fn exit_if_errors(&self) {
        if self.errors.get() > 0 {
            panic!("Exiting due to previous fatal errors.")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RamenSource {}
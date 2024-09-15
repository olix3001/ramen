use std::cell::{Cell, RefCell};

use hashbrown::HashMap;
use slotmap::SlotMap;

use crate::{ast::NodeId, error::{Diagnostic, ResolutionError}, scope::ScopeMapRef};

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
}

impl Session {
    pub fn new() -> Self {
        Self {
            sources: RefCell::default(),
            errors: Cell::new(0),

            scopes: ScopeMapRef::new(),
            refs: RefCell::default()
        }
    }

    // ==< Ref-related >==
    pub fn get_ref_target(&self, id: NodeId) -> Result<NodeId, ResolutionError> {
        match self.refs.borrow().get(&id) {
            Some(def_id) => Ok(*def_id),
            None => todo!("Throw proper resolution error")
        }
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
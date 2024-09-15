use session::SourceId;
use slotmap::Key;

pub mod error;
pub mod session;
pub mod ast;
pub mod visitor;
pub mod scope;

pub extern crate ariadne;

/// Location of given element in source code including span and file
#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub span: core::ops::Range<usize>,
    pub file: SourceId
}

impl Loc {
    pub fn new(file: SourceId, span: core::ops::Range<usize>) -> Self {
        Self { file, span }
    }
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}@<{}>", self.span.start, self.span.end, self.file.data().as_ffi())
    }
}
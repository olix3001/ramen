use crate::{session::{Session, SourceId}, Loc};

impl ariadne::Span for Loc {
    type SourceId = SourceId;

    fn source(&self) -> &Self::SourceId { &self.file }

    fn start(&self) -> usize { self.span.start }
    fn end(&self) -> usize { self.span.end }
}

pub trait Diagnostic {
    fn is_fatal(&self) -> bool;
    fn get_location(&self) -> Loc;
    fn build_report(&self, session: &Session) -> ariadne::Report<'static, Loc>;
}

#[derive(Debug, Clone)]
pub enum ResolutionError {

}

impl Diagnostic for ResolutionError {
    fn is_fatal(&self) -> bool { true }

    fn get_location(&self) -> Loc {
        todo!()
    }

    fn build_report(&self, session: &Session) -> ariadne::Report<'static, Loc> {
        todo!()
    }
}
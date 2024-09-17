use ramen_common::{ast::{self, NodeId}, error::Diagnostic, scope::ScopeRef, session::Session};

pub mod binding;
pub mod type_resolution;

pub trait ASTPass<'sess, R> {
    type Error: Diagnostic;

    fn run_on_module(session: &'sess Session, scope: ScopeRef, mod_id: NodeId, module: &ast::Module) -> Result<R, Self::Error>;
}
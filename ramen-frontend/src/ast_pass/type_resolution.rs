use ramen_common::{ast::{self, NodeId}, error::ResolutionError, scope::ScopeRef, session::Session, types::RamenType, visitor::{walk_parameter, ScopeStack, Visitor}};

use super::ASTPass;

pub struct TypeResolutionPass<'sess> {
    pub session: &'sess Session,
    pub stack: ScopeStack
}

impl<'sess> ASTPass<'sess, ()> for TypeResolutionPass<'sess> {
    type Error = ResolutionError;

    fn run_on_module(session: &'sess Session, scope: ScopeRef, mod_id: NodeId, module: &ast::Module) -> Result<(), Self::Error> {
        let mut name_binder = Self {
            session,
            stack: ScopeStack::new()
        };

        name_binder.stack.push_scope(scope);
        name_binder.visit_module(mod_id, module)?;

        session.exit_if_errors();
        Ok(())
    }
}

impl<'sess> Visitor<()> for TypeResolutionPass<'sess> {
    type Error = ResolutionError;

    fn default_return(&self) -> () { () }
    fn get_scope_stack<'a>(&'a self) -> &'a ScopeStack { &self.stack }
    fn get_session<'a>(&'a self) -> &'a Session { &self.session }

    fn visit_value_parameter(&mut self, parameter: &ast::ValueParameter) -> Result<(), Self::Error> {
        self.visit_parameter(&parameter.parameter)?;
        self.session.try_bind_type(parameter.id, parameter.parameter.id);
        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &ast::Parameter) -> Result<(), Self::Error> {
        walk_parameter(self, parameter)?;
        self.session.try_bind_type(parameter.id, parameter.ty.id);
        Ok(())
    }

    fn visit_type(&mut self, ty: &ast::Type) -> Result<(), Self::Error> {
        let resolved_type = match &ty.kind {
            ast::TypeKind::Unit => RamenType::Unit,
            ast::TypeKind::Integer(width) => RamenType::Integer(*width)
        };

        self.session.set_type(ty.id, resolved_type);
        Ok(())
    }
}
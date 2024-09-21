use ramen_common::{ast::{self, NodeId}, error::ResolutionError, scope::ScopeRef, session::Session, types::{CallableType, RamenType}, visitor::{walk_function, walk_parameter, ASTPass, ScopeStack, Visitor}};

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

    fn visit_function(&mut self, id: NodeId, function: &ast::Function) -> Result<(), Self::Error> {
        let def_id = self.session.get_ref_target(id).expect("Cannot find function definition reference.");
        walk_function(self, id, function)?;

        let mut parameter_types = Vec::new();
        for parameter in function.parameters.iter() {
            let parameter_type = self.session.get_type(parameter.id);
            parameter_types.push(parameter_type.unwrap());
        }

        let return_type = if let Some(return_type) = &function.return_type {
            self.session.get_type(return_type.id).unwrap()
        } else { RamenType::Unit };

        self.session.set_type(def_id, RamenType::Callable(Box::new(CallableType::new(
            return_type,
            parameter_types
        ))));
        self.session.try_bind_type(id, def_id);

        Ok(())
    }

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

    fn visit_literal_expression(&mut self, id: NodeId, literal: &ast::Literal) -> Result<(), Self::Error> {
        match literal {
            ast::Literal::Integer(_) => self.session.set_type(id, RamenType::Integer(32)), // Default width... change to minimum required in the future
        } 

        Ok(())
    }

    fn visit_type(&mut self, ty: &ast::Type) -> Result<(), Self::Error> {
        let resolved_type = match &ty.kind {
            ast::TypeKind::Unit => RamenType::Unit,
            ast::TypeKind::Integer(width) => RamenType::Integer(*width),
        };

        self.session.set_type(ty.id, resolved_type);
        Ok(())
    }
}
use ramen_common::{ast::{self, NodeId}, defs::Definition, error::ResolutionError, scope::ScopeRef, session::Session, visitor::{walk_function, walk_module, ASTPass, ScopeStack, Visitor}};

pub struct ItemNameBindingPass<'sess> {
    pub session: &'sess Session,
    pub stack: ScopeStack
}

impl<'sess> ASTPass<'sess, ()> for ItemNameBindingPass<'sess> {
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

impl<'sess> Visitor<()> for ItemNameBindingPass<'sess> {
    type Error = ResolutionError;

    fn default_return(&self) -> () { () }
    fn get_scope_stack<'a>(&'a self) -> &'a ScopeStack { &self.stack }
    fn get_session<'a>(&'a self) -> &'a Session { &self.session }

    fn visit_module(&mut self, id: NodeId, module: &ast::Module) -> Result<(), Self::Error> {
        let module_def_id = self.session.alloc_def(id);
        self.session.set_def(module_def_id, Definition::Module);
        self.session.scopes.add(module_def_id, Some(self.stack.get_scope()), Some(module.name.clone()));
        self.stack.get_scope().define_name(&module.name, module_def_id);

        walk_module(self, id, module)
    }

    fn visit_function(&mut self, id: NodeId, function: &ast::Function) -> Result<(), Self::Error> {
        let function_def_id = self.session.alloc_def(id);
        self.session.set_def(function_def_id, Definition::Function);
        self.session.scopes.add(function_def_id, Some(self.stack.get_scope()), Some(function.name.clone()));
        self.stack.get_scope().define_name(&function.name, function_def_id);
        self.session.set_symbol(id, self.stack.prefix_name(".", &function.name));

        walk_function(self, id, function)
    }
}
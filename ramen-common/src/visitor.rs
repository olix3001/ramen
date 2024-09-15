use std::cell::RefCell;

use crate::{ast::{self, NodeId}, error::Diagnostic, scope::ScopeRef, session::Session};

#[derive(Debug, Clone)]
pub struct ScopeStack {
    stack: RefCell<Vec<ScopeRef>>
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            stack: RefCell::default()
        }
    }

    pub fn push_scope(&self, scope: ScopeRef) {
        self.stack.borrow_mut().push(scope)
    }

    pub fn pop_scope(&self) {
        self.stack.borrow_mut().pop();
    }

    pub fn get_scope(&self) -> ScopeRef {
        self.stack.borrow().last().cloned().expect("Scope stack should have at least one scope.")
    }
}

pub trait Visitor<T> where Self: Sized {
    type Error: Diagnostic;

    fn default_return(&self) -> T;

    fn get_scope_stack<'a>(&'a self) -> &'a ScopeStack;
    fn get_session<'a>(&'a self) -> &'a Session;

    fn with_scope<F>(&mut self, scope: ScopeRef, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        self.get_scope_stack().push_scope(scope);
        let temp = f(self);
        self.get_scope_stack().pop_scope();
        temp
    }

    // ==< Items >==
    fn visit_item(&mut self, item: &ast::Item) -> Result<T, Self::Error> { walk_item(self, item) }
    fn visit_item_stream(&mut self, stream: &Vec<ast::Item>) -> Result<T, Self::Error> { walk_item_stream(self, stream) }

    fn visit_module(&mut self, id: NodeId, module: &ast::Module) -> Result<T, Self::Error> { walk_module(self, id, module) }
    fn visit_function(&mut self, id: NodeId, function: &ast::Function) -> Result<T, Self::Error> { walk_function(self, id, function) }

    // ==< Statements >==
    fn visit_statement(&mut self, statement: &ast::Statement) -> Result<T, Self::Error> { walk_statement(self, statement) }
    fn visit_statement_stream(&mut self, stream: &Vec<ast::Statement>) -> Result<T, Self::Error> { walk_statement_stream(self, stream) }

    // ==< Expressions >==
    fn visit_expression(&mut self, expression: &ast::Expression) -> Result<T, Self::Error> { walk_expression(self, expression) }

    fn visit_literal_expression(&mut self, _literal: &ast::Literal) -> Result<T, Self::Error> { Ok(self.default_return()) }

    // ==< Types >==
    fn visit_type(&mut self, ty: &ast::Type) -> Result<T, Self::Error> { walk_type(self, ty) }

    // ==< Other/Utility >==
    fn visit_value_parameter(&mut self, parameter: &ast::ValueParameter) -> Result<T, Self::Error> { walk_value_parameter(self, parameter) }
    fn visit_parameter(&mut self, parameter: &ast::Parameter) -> Result<T, Self::Error> { walk_parameter(self, parameter) }

    fn visit_block(&mut self, block: &ast::Block) -> Result<T, Self::Error> { walk_block(self, block) }
}

// ==< Items >==
pub fn walk_item<V, T>(visitor: &mut V, item: &ast::Item) -> Result<T, V::Error>
where V: Visitor<T> {
    match &item.kind {
        ast::ItemKind::Module(module) => visitor.visit_module(item.id, module),
        ast::ItemKind::Function(function) => visitor.visit_function(item.id, function),
    }
}

pub fn walk_item_stream<V, T>(visitor: &mut V, stream: &Vec<ast::Item>) -> Result<T, V::Error>
where V: Visitor<T> {
    for item in stream.iter() {
        visitor.visit_item(item)?;
    }
    Ok(visitor.default_return())
}

pub fn walk_module<V, T>(visitor: &mut V, id: NodeId, module: &ast::Module) -> Result<T, V::Error>
where V: Visitor<T> {
    let def_id = visitor.get_session().get_ref_target(id).expect("Cannot find module definition reference.");
    let module_scope = visitor.get_session().scopes.get(def_id).expect("Cannot find module scope.");

    visitor.with_scope(module_scope, |visitor|
        visitor.visit_item_stream(&module.items)
    )
}

pub fn walk_function<V, T>(visitor: &mut V, id: NodeId, function: &ast::Function) -> Result<T, V::Error>
where V: Visitor<T> {
    let def_id = visitor.get_session().get_ref_target(id).expect("Cannot find function definition reference.");
    let function_scope = visitor.get_session().scopes.get(def_id).expect("Cannot find function scope.");

    visitor.with_scope(function_scope, |visitor| { 
        for parameter in function.parameters.iter() {
            visitor.visit_value_parameter(parameter)?;
        }

        if let Some(return_type) = &function.return_type {
            visitor.visit_type(&return_type)?;
        }
        visitor.visit_block(&function.body)?;
        Ok(visitor.default_return())
    })?;

    Ok(visitor.default_return())
}

// ==< Statements >==
pub fn walk_statement<V, T>(visitor: &mut V, statement: &ast::Statement) -> Result<T, V::Error>
where V: Visitor<T> {
    match &statement.kind {
        ast::StatementKind::Item(item) => visitor.visit_item(item),
        ast::StatementKind::Expression(expression) => visitor.visit_expression(expression),
    }
}

pub fn walk_statement_stream<V, T>(visitor: &mut V, stream: &Vec<ast::Statement>) -> Result<T, V::Error>
where V: Visitor<T> {
    for item in stream.iter() {
        visitor.visit_statement(item)?;
    }
    Ok(visitor.default_return())
}

// ==< Expressions >==
pub fn walk_expression<V, T>(visitor: &mut V, expression: &ast::Expression) -> Result<T, V::Error>
where V: Visitor<T> {
    match &expression.kind {
        ast::ExpressionKind::Literal(literal) => visitor.visit_literal_expression(literal),
    }
}

// ==< Types >==
pub fn walk_type<V, T>(visitor: &mut V, _ty: &ast::Type) -> Result<T, V::Error>
where V: Visitor<T> {
    Ok(visitor.default_return())
}

// ==< Other/Utility >==
pub fn walk_value_parameter<V, T>(visitor: &mut V, parameter: &ast::ValueParameter) -> Result<T, V::Error>
where V: Visitor<T> {
    visitor.visit_parameter(&parameter.parameter)?;
    if let Some(initializer) = &parameter.initializer {
        visitor.visit_expression(initializer)?;
    }
    Ok(visitor.default_return())
}

pub fn walk_parameter<V, T>(visitor: &mut V, parameter: &ast::Parameter) -> Result<T, V::Error>
where V: Visitor<T> {
    visitor.visit_type(&parameter.ty)?;
    Ok(visitor.default_return())
}

pub fn walk_block<V, T>(visitor: &mut V, block: &ast::Block) -> Result<T, V::Error>
where V: Visitor<T> {
    visitor.visit_statement_stream(&block.statements)
}
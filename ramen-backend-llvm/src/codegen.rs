use inkwell::{builder::Builder, context::Context, module::Module, values::{AnyValue, AnyValueEnum, BasicValueEnum}};
use ramen_common::{ast::{self, NodeId}, scope::ScopeRef, session::Session, visitor::{walk_expression, walk_function, ASTPass, ScopeStack, Visitor}};

use crate::{error::CodegenError, types::AsLLType};

pub struct LLVMBackendCodegenPass<'sess, 'ctx> {
    pub session: &'sess Session,
    pub stack: ScopeStack,

    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx> 
}

pub fn generate_llvm_module(session: &Session, scope: ScopeRef, mod_id: NodeId, module: &ast::Module) -> Result<(), CodegenError> {
    let context = Context::create();
    let mut codegen = LLVMBackendCodegenPass {
        session,
        stack: ScopeStack::new(),

        context: &context,
        module: context.create_module(&module.name),
        builder: context.create_builder()
    };

    codegen.stack.push_scope(scope);
    codegen.visit_module(mod_id, module)?;

    println!("Finished codegen:\n{}", codegen.module.print_to_string());

    session.exit_if_errors();
    Ok(())
}

type VisitorReturn<'ctx> = Option<AnyValueEnum<'ctx>>;
impl<'sess, 'ctx> Visitor<VisitorReturn<'ctx>> for LLVMBackendCodegenPass<'sess, 'ctx> {
    type Error = CodegenError;

    fn default_return(&self) -> VisitorReturn<'ctx> { None }
    fn get_scope_stack<'a>(&'a self) -> &'a ScopeStack { &self.stack }
    fn get_session<'a>(&'a self) -> &'a Session { &self.session }

    fn visit_function(&mut self, id: NodeId, function: &ast::Function) -> Result<VisitorReturn<'ctx>, Self::Error> {
        let ll_function = self.module.add_function(
            &self.stack.prefix_name("$", &function.name),
            self.session.get_type(id)
                .expect("Function type should have been resolved by frontend")
                .as_llvm_type(&self.context)?.into_function_type(),
            None // TODO: Replace with linkage based on modifiers like extern "abi".
        );
        
        let basic_block = self.context.append_basic_block(ll_function, "entry");
        self.builder.position_at_end(basic_block);
        walk_function(self, id, function)?;
        self.builder.clear_insertion_position(); // temporary.
        Ok(None)
    }

    fn visit_return_statement(&mut self, _id: NodeId, value: &ast::Expression) -> Result<VisitorReturn<'ctx>, Self::Error> {
        let return_value: BasicValueEnum = walk_expression(self, value)?
            .expect("Temporary unwrap, this will error readably later on")
            .try_into().map_err::<CodegenError, _>(|_| todo!())?;
        self.builder.build_return(Some(&return_value))?;
        Ok(None)
    }

    fn visit_literal_expression(&mut self, id: NodeId, literal: &ast::Literal) -> Result<VisitorReturn<'ctx>, Self::Error> {
        match &literal {
            ast::Literal::Integer(value) => {
                let int_type = self.session.get_type(id).expect("This should have been set by type resolution/checking");
                let value = int_type.as_llvm_type(&self.context).unwrap().into_int_type().const_int(*value as _, true); // temporary.
                Ok(Some(AnyValueEnum::IntValue(value)))
            }
        }
    }
}
use ramen_common::{ast::NodeId, scope::Scope, session::{Session, SourceId}, visitor::ASTPass};
use ramen_frontend::{lex, parse, ast_pass::{binding, type_resolution}};

#[test]
fn compile_function() {
    const SOURCE: &'static str = "func identity(a: int32): int32 => 15";
    let mut tokens = lex::Tokens::from_string(SOURCE, SourceId::dummy());
    let ast = parse::parse_ramen("main".to_string(), &mut tokens).expect("Something went wrong during parsing");

    let session = Session::new();
    let module_id = NodeId::next();
    let global_scope = Scope::new_ref(None, None);

    binding::ItemNameBindingPass::run_on_module(&session, global_scope.clone(), module_id, &ast)
        .expect("Something went wrong during item name binding pass.");

    type_resolution::TypeResolutionPass::run_on_module(&session, global_scope.clone(), module_id, &ast)
        .expect("Something went wrong during type resolution pass.");

    ramen_backend_llvm::codegen::generate_llvm_module(&session, global_scope.clone(), module_id, &ast)
        .expect("Failed to generate llvm module from AST.");

    panic!()
}
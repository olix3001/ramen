pub mod lex;
pub mod parse;
pub mod error;
pub mod ast_pass;

#[cfg(test)]
mod test {
    use logos::Logos;

    use crate::{ast_pass::{binding::ItemNameBindingPass, type_resolution::TypeResolutionPass, ASTPass}, lex::{self, Token}, parse};
    use ramen_common::{ast::NodeId, scope::Scope, session::{Session, SourceId}};

    #[test]
    fn lex_func() {
        const SOURCE: &'static str = "func identity(a: int32) => 15";
        let mut tokens = lex::Tokens::from_lexer(Token::lexer(SOURCE), SourceId::dummy());
        let ast = parse::parse_ramen("main".to_string(), &mut tokens).expect("Something went wrong during parsing");

        let session = Session::new();
        let module_id = NodeId::next();
        let global_scope = Scope::new_ref(None);

        ItemNameBindingPass::run_on_module(&session, global_scope.clone(), module_id, &ast)
            .expect("Something went wrong during item name binding pass.");

        TypeResolutionPass::run_on_module(&session, global_scope.clone(), module_id, &ast)
            .expect("Something went wrong during type resolution pass.");

        panic!("{session:?}");
    }
}
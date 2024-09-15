pub mod lex;
pub mod parse;
pub mod error;

#[cfg(test)]
mod test {
    use logos::Logos;

    use crate::{lex::{self, Token}, parse};
    use ramen_common::session::SourceId;

    #[test]
    fn lex_func() {
        let SOURCE = "func identity(a: int32) => 15";
        let mut tokens = lex::Tokens::from_lexer(Token::lexer(SOURCE), SourceId::dummy());
        let ast = parse::parse_ramen("main".to_string(), &mut tokens);

        panic!("{ast:#?}");
    }
}
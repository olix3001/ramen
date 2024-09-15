pub mod lex;
pub mod error;

#[cfg(test)]
mod test {
    use logos::Logos;

    use crate::lex::{self, Token};
    use ramen_common::session::SourceId;

    #[test]
    fn lex_func() {
        let SOURCE = "func identity(a: int32) = a";
        let mut tokens = lex::Tokens::from_lexer(Token::lexer(SOURCE), SourceId::dummy());
        for _ in 0..9 {
            println!("{:?}", tokens.next());
        }
    }
}
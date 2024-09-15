pub mod lex;

#[cfg(test)]
mod test {
    use logos::Logos;

    use crate::lex::{self, Token};

    #[test]
    fn lex_func() {
        let SOURCE = "func identity(a: int32) = a";
        let mut tokens = lex::Tokens::from_lexer(Token::lexer(SOURCE));
        for _ in 0..9 {
            println!("{:?}", tokens.next());
        }
    }
}
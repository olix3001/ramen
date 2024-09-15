use logos::{Lexer, Logos};
use ramen_common::{Loc, session::SourceId};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"[ \t\r\f]+")] // Do not skip newline as it acts as semicolon
pub enum Token {
    // ==< Operators & Separators >==
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[token("[")] LeftBracket,
    #[token("]")] RightBracket,
    #[token("{")] LeftCurly,
    #[token("}")] RightCurly,
    #[token("<")] LeftAngle,
    #[token(">")] RightAngle,

    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token(".")] Dot,
    #[token(",")] Comma,
    #[token("=")] Assign,
    
    #[token("++")] Increment,
    #[token("--")] Decrement,
    #[token("&&")] Conjunction,
    #[token("||")] Disjunction,
    #[token("+=")] AddAssign,
    #[token("-=")] SubAssign,
    #[token("*=")] MulAssign,
    #[token("/=")] DivAssign,
    #[token("%=")] ModAssign,

    #[token("->")] Arrow,
    #[token("=>")] FatArrow,
    #[token("..")] Range,
    #[token("..=")] RangeInclusive,
    #[token("...")] Spread,
    #[token("#")] Hash,
    #[token("@")] At,
    #[token("&")] Ampersand,
    #[token("|")] Pipe,

    #[token("<=")] LessEqual,
    #[token(">=")] GreaterEqual,
    #[token("!=")] NotEqual,
    #[token("==")] EqualEqual,

    // ==< Keywords >==
    #[token("func")] FuncKW,

    // ==< Type literals >==
    #[regex(r"int[0-9]+")] IntegerType,

    // ==< Value literals >==

    // ==< Modifiers >==

    // ==< Other >==
    #[token("\n")] NL,
    #[regex(r"[_\p{L}][_\p{L}\p{N}]*")] Identifier
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo(pub(crate) Token, pub(crate) String, pub(crate) Loc);

#[derive(Debug, Clone)]
pub struct Tokens<'src> {
    iter: Lexer<'src, Token>,
    stack: Vec<TokenInfo>,
    current: usize,
    source: SourceId
}

impl<'src> Tokens<'src> {
    pub fn from_lexer(iter: Lexer<'src, Token>, source: SourceId) -> Self {
        Self {
            iter,
            stack: Vec::new(),
            current: 0,
            source
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.current == self.stack.len() {
            let Some(Ok(next_elem)) = self.iter.next() else { return None; };

            let slice = self.iter.slice().to_string();
            let range = self.iter.span();

            self.stack.push(TokenInfo(next_elem, slice, Loc::new(self.source, range)));
            self.current += 1;
            Some(next_elem)
        } else {
            if self.current >= self.stack.len() {
                return None;
            }
            self.current += 1;
            Some(self.stack[self.current - 1].0)
        }
    }

    pub fn current(&self) -> Option<Token> {
        if self.current < 1 || self.current - 1 > self.stack.len() 
            { return None; }
        Some(self.stack[self.current - 1].0)
    }

    pub fn back(&mut self) -> bool {
        if self.current < 1 { return false }
        self.current -= 1;
        true
    }

    pub fn peek(&mut self) -> Option<Token> {
        let next = self.next();
        if next.is_some() {
            self.current -= 1;
        }
        next
    }

    pub fn is(&mut self, token: Token) -> bool {
        let next = self.next();
        if next != Some(token) {
            self.back();
            false
        } else { true }
    }

    pub fn is_any(&mut self, tokens: &[Token]) -> Option<Token> {
        let next = self.next();
        if let Some(next) = next {
            if tokens.contains(&next) { return Some(next) }
        }
        self.back();
        None
    }

    pub fn loc(&self) -> Option<Loc> {
        if self.current < 1 || self.current - 1 > self.stack.len() 
            { return None; }
        Some(self.stack[self.current - 1].2.clone())
    }

    pub fn text(&self) -> Option<&str> {
        if self.current < 1 || self.current - 1 > self.stack.len() 
            { return None; }
        Some(self.stack[self.current - 1].1.as_str())
    }
}
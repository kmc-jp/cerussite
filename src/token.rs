use regex::Regex;

macro_rules! define_tokens {
    ($(literal $lname:ident: $lmatcher:expr;)* $(regex ($rregex_name:ident) $rname:ident: $rmatcher:expr;)*) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum Token<'a> {
            $($lname,)*
            $($rname(&'a str),)*
        }

        impl<'a> Token<'a> {
            pub fn from_str(token_str: &'a str) -> Option<Token<'a>> {
                match token_str {
                    $(
                        $lmatcher => return Some(Token::$lname),
                    )*
                    _ => {},
                }

                $(
                    if $rregex_name.is_match(token_str) {
                        return Some(Token::$rname(token_str));
                    }
                )*

                None
            }
        }

        lazy_static! {
            $(
                static ref $rregex_name: Regex = Regex::new($rmatcher).unwrap();
            )*
        }
    };
}

define_tokens! {
    literal TyInt: "int";
    literal TyVoid: "void";

    literal KwReturn: "return";

    literal OpAdd: "+";
    literal OpSub: "-";
    literal OpMul: "*";
    literal OpDiv: "/";
    literal OpRem: "%";

    literal OpAssign: "=";

    literal SyLPar: "(";
    literal SyRPar: ")";
    literal SyLBrace: "{";
    literal SyRBrace: "}";
    literal SySemicolon: ";";

    regex (RE_IDENT) Ident: "^[a-zA-Z_][a-zA-Z0-9_]*$";
    regex (RE_LITERAL) Literal: "^[0-9]+$";
}

#[derive(Debug, Clone, Copy)]
pub struct Tokens<'a> {
    tokens: &'a [Token<'a>],
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Tokens<'a> {
        Tokens { tokens }
    }
}

impl<'a> Tokens<'a> {
    pub fn next(&mut self) -> Option<Token<'a>> {
        let res = self.peek();
        if res.is_some() {
            self.tokens = &self.tokens[1..];
        }
        res
    }

    pub fn peek(&self) -> Option<Token<'a>> {
        self.tokens.iter().next().cloned()
    }

    pub fn eat(&mut self, expect: Token<'a>) {
        self.eat_impl(expect, None);
    }

    pub fn eat_err(&mut self, expect: Token<'a>, msg_if_fails: &str) {
        self.eat_impl(expect, Some(msg_if_fails));
    }

    fn eat_impl(&mut self, expect: Token<'a>, msg_if_fails: Option<&str>) {
        let msg_if_fails = msg_if_fails.unwrap_or("ICE: unexpected token");
        let token = self.next();
        assert_eq!(token, Some(expect), "{}", msg_if_fails);
    }

    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokens() {
        let tokens = Tokens::new(&[]);
        assert!(tokens.is_empty());
        let mut tokens = Tokens::new(&[Token::TyVoid]);
        assert!(!tokens.is_empty());
        assert_eq!(Some(Token::TyVoid), tokens.peek());
        tokens.eat(Token::TyVoid);
        assert!(tokens.is_empty());
    }
}

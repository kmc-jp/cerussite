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

    literal SyLPar: "(";
    literal SyRPar: ")";
    literal SyLBrace: "{";
    literal SyRBrace: "}";
    literal SySemicolon: ";";

    regex (RE_IDENT) Ident: "^[a-zA-Z_][a-zA-Z0-9_]*$";
    regex (RE_LITERAL) Literal: "^[0-9]+$";
}

pub type Tokens<'a> = &'a [Token<'a>];

impl<'a> Token<'a> {
    pub fn unwrap_literal(&self) -> &str {
        match *self {
            Token::Literal(n) => n,
            _ => panic!("expected literal, found {:?}", self),
        }
    }
}

use crate::lexer;

const POSTFIX: &[&str] = &["!", "?", "+", "-"];
const PREFIX: &[&str] = &["!", "?"];

pub enum Expr {
    String(String),
    Variable(String),
    Const(String),

    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>
    },

    Unary {
        oper: Box<Expr>,
        op: String
    },

    Func {
        name: String,
        args: Vec<Expr>
    },

    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_then: Box<Expr>
    },

    While {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_then: Box<Expr>
    },

    For {
        iter: usize,
        var: String,
        then: Box<Expr>,
        else_then: Box<Expr>
    },

    Define {
        var: String,
        val: Box<Expr>,
    },

    Block(Vec<Expr>)
}


fn precedence(token: &lexer::Token) -> usize {
    let v = &token.value;

    match v {
        Some(lexer::TokenValue::Str(s)) => match s.as_str() {
            "^^" => return 4,   // Tetration
            "^" => return 3,    // Exponention
            "*" | "/" => return 2,  // Multiplication/Division
            "+" | "-" => return 1,  // Addition/Subtraction
            ">" | "<" | "=" | "≥" | "≤" => return 0,    // Comparisons
            _ => crate::fail()
        }
        _ => crate::fail()
    }
}

pub struct Parser {
    tokens: Vec<lexer::Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>, pos: usize) -> Self {
        Self { tokens, pos }
    }

    fn peek(&self) -> Option<&lexer::Token> {
        return self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<lexer::Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    fn expect(&mut self, types: &[lexer::TokenType]) -> lexer::Token {
        let token = match self.advance() {
            Some(t) => t,
            None => crate::fail()
        };
        if !types.iter().any(|t| *t == token._type) {
            crate::fail();
        }
        return token;
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(lexer::Token {_type: lexer::TokenType::Const, value: Some(lexer::TokenValue::Str(s))}) => return Expr::Const(s.clone()),
            Some(lexer::Token {_type: lexer::TokenType::Variable, value: Some(lexer::TokenValue::Str(s))}) => return Expr::Variable(s.clone()),
            Some(lexer::Token {_type: lexer::TokenType::String, value: Some(lexer::TokenValue::Str(s))}) => {
                if self.peek().is_some() && self.peek().unwrap()._type == lexer::TokenType::LeftParen {
                    let mut args = Vec::<Expr>::new();
                    self.expect(&[lexer::TokenType::LeftParen]);
                    while let Some(token) = self.peek() {
                        if token._type == lexer::TokenType::RightParen {
                            break;
                        }
                        args.push(self.parse_primary());
                    }
                    self.expect(&[lexer::TokenType::RightParen]);
                    return Expr::Func { name: s.clone(), args };
                } else {
                    return Expr::String(s.clone());
                }
            },
            Some(lexer::Token {_type: lexer::TokenType::Define, .. }) => {
                let var = match self.expect(&[lexer::TokenType::Const, lexer::TokenType::Variable]).value {
                    Some(lexer::TokenValue::Str(s)) => s.clone(),
                    _ => crate::fail()
                };

                self.expect(&[lexer::TokenType::Assign]);
                let val = self.parse_primary();
                self.expect(&[lexer::TokenType::EndOfAssign]);
                return Expr::Define { var, val: Box::new(val)};
            },
            Some(lexer::Token {_type: lexer::TokenType::LeftParen, .. }) => {
                let expr = self.parse_expr();
                self.expect(&[lexer::TokenType::RightParen]);
                return expr;
            },
            Some(lexer::Token {_type: lexer::TokenType::If, .. }) => {
                return self.parse_if()
            },
            Some(lexer::Token {_type: lexer::TokenType::For, .. }) => {
                return self.parse_for()
            },
            _ => crate::fail()
        }
    }

    fn parse_expr(&mut self) -> Expr {
        return self.parse_binary(0);
    }

    fn parse_binary(&mut self, min_prec: usize) -> Expr {
        let mut left = self.parse_prefix();

        while let Some(token) = self.peek() {
            if token._type != lexer::TokenType::BinaryOperator {
                break;
            }

            let prec = precedence(token);
            if prec < min_prec {
                break;
            }

            let op_token = self.advance().unwrap();
            let right = self.parse_binary(prec + 1);

            left = Expr::Binary {
                left: Box::new(left),
                op: match &op_token.value {
                    Some(lexer::TokenValue::Str(s)) => s.clone(),
                    _ => crate::fail()
                },
                right: Box::new(right),
            };
        }

        return left;
    }

    fn parse_prefix(&mut self) -> Expr {
        if let Some(token) = self.peek() {
            if let Some(lexer::TokenValue::Char(c)) = &token.value {
                if PREFIX.contains(&c.to_string().as_str()) {
                    let op = c.to_string();
                    self.advance();
                    let expr = self.parse_prefix();
                    return Expr::Unary {
                        oper: Box::new(expr),
                        op,
                    };
                }
            }
        }

        return self.parse_postfix();
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            let token = match self.peek() {
                Some(t) => t,
                None => break,
            };

            match token._type {
                _ if matches!(token.value, Some(lexer::TokenValue::Char(c))
                    if POSTFIX.contains(&c.to_string().as_str())) =>
                {
                    let op = match &token.value {
                        Some(lexer::TokenValue::Char(c)) => c.to_string(),
                        _ => unreachable!(),
                    };
                    self.advance();
                    expr = Expr::Unary {
                        oper: Box::new(expr),
                        op,
                    };
                }

                lexer::TokenType::While => {
                    self.advance();

                    let then_branch = self.parse_expr();
                    self.expect(&[lexer::TokenType::Else]);
                    let else_branch = self.parse_expr();

                    expr = Expr::While {
                        cond: Box::new(expr),
                        then: Box::new(then_branch),
                        else_then: Box::new(else_branch),
                    };
                }

                _ => break,
            }
        }

        return expr
    }

    fn parse_if(&mut self) -> Expr {
        let cond = self.parse_expr();
        let then_branch = self.parse_expr();

        self.expect(&[lexer::TokenType::Else]);
        let else_branch = self.parse_expr();

        Expr::If {
            cond: Box::new(cond),
            then: Box::new(then_branch),
            else_then: Box::new(else_branch),
        }
    }

    fn parse_for(&mut self) -> Expr {
        let token = self.tokens.get(self.pos - 1).unwrap();

        let iter = match &token.value {
            Some(lexer::TokenValue::Str(s)) => {
                s.parse::<usize>().unwrap_or_else(|_| crate::fail())
            }
            _ => crate::fail()
        };

        let var_token = self.expect(&[lexer::TokenType::Variable]);

        let var_name = match &var_token.value {
            Some(lexer::TokenValue::Str(s)) => s.clone(),
            _ => crate::fail()
        };

        let then_branch = self.parse_expr();

        self.expect(&[lexer::TokenType::Else]);
        let else_branch = self.parse_expr();

        Expr::For {
            iter,
            var: var_name,
            then: Box::new(then_branch),
            else_then: Box::new(else_branch),
        }
    }
}




impl Parser {
    pub fn parse(&mut self) -> Expr {
        let mut exprs = Vec::new();

        while self.peek().is_some() {
            if self.tokens.get(self.pos())._type == lexer::TokenType::Newline {
                self.advance();
                continue;
            };
            exprs.push(self.parse_expr());
        }

        Expr::Block(exprs)
    }
}

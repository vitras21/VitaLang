use crate::lexer;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Expr {
    String(String),
    Array(Vec<String>),
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

    Try {
        attempt: Box<Expr>,
        catch: Box<Expr>
    },

    Yield(Box<Expr>),

    Break(),

    Block(Vec<Expr>)
}

pub struct Parser {
    tokens: Vec<lexer::Token>,
    pos: usize,
    precedence_map: HashMap<String, usize>,
    prefix_ops: HashSet<String>,
    postfix_ops: HashSet<String>,
}

impl Parser {
    pub fn new(
        tokens: Vec<lexer::Token>,
        pos: usize,
        precedence_map: HashMap<String, usize>,
        operator_defs: Vec<crate::first_pass::OperatorDef>,
    ) -> Self {
        let mut prefix_ops: HashSet<String> =
            ["!", "?"].iter().map(|s| s.to_string()).collect();
        let mut postfix_ops: HashSet<String> =
            ["!", "?"].iter().map(|s| s.to_string()).collect();

        postfix_ops.insert("++".to_string());

        for def in &operator_defs {
            match def.kind {
                crate::first_pass::OperatorKind::Prefix => {
                    prefix_ops.insert(def.op.clone());
                }
                crate::first_pass::OperatorKind::Postfix => {
                    postfix_ops.insert(def.op.clone());
                }
                crate::first_pass::OperatorKind::Binary => {}
            }
        }

        Self {
            tokens,
            pos,
            precedence_map,
            prefix_ops,
            postfix_ops,
        }
    }

    fn precedence(&self, token: &lexer::Token) -> usize {
        let v = &token.value;

        match v {
            Some(lexer::TokenValue::Str(s)) =>  return *self.precedence_map.get(s.as_str()).unwrap_or(&0),
            _ => crate::fail()
        }
    }

    fn peek(&self) -> Option<&lexer::Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<lexer::Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    fn expect(&mut self, types: &[lexer::TokenType]) -> lexer::Token {
        let token = match self.advance(){ Some(t) => t, None => crate::fail(), };
        if !types.iter().any(|t| *t == token._type) {
            panic!(
                "parse error at pos {}: expected one of {:?}, got {:?}",
                self.pos.saturating_sub(1),
                types,
                token._type
            );
        }
        token
    }

    pub fn parse(&mut self) -> Expr {
        // eprintln!("[DEBUG] Starting parse with {} tokens", self.tokens.len());
        let mut exprs = Vec::new();

        while let Some(token) = self.peek() {
            match token._type {
                lexer::TokenType::EOF => break,

                lexer::TokenType::Newline | lexer::TokenType::Comment | lexer::TokenType::BlockCommentStart | lexer::TokenType::BlockCommentEnd => {
                    self.advance();
                }

                lexer::TokenType::Indent | lexer::TokenType::Dedent => {
                    crate::fail();
                }

                _ => exprs.push(self.parse_expr()),
            }
        }

        // eprintln!("[DEBUG] Parse complete, {} expressions", exprs.len());
        Expr::Block(exprs)
    }

    fn parse_expr(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_expr with {}", self.tokens.get(self.pos).unwrap());
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_prec: usize) -> Expr {
        // eprintln!("[DEBUG] parse_binary(min_prec={}) with {}", min_prec, self.tokens.get(self.pos).unwrap());
        let mut left = self.parse_prefix();

        while let Some(token) = self.peek() {
            
            if token._type != lexer::TokenType::BinaryOperator {
                // eprintln!("[DEBUG] Not a binary operator, breaking");
                break;
            }

            if let Some(op) = token_op(token) {
                if self.postfix_ops.contains(&op) {
                    break;
                }
            }

            let prec = self.precedence(token);
            if prec < min_prec {
                // eprintln!("[DEBUG] Precedence {} < {}, breaking", prec, min_prec);
                break;
            }

            let op_token = self.advance().unwrap();
            // eprintln!("[DEBUG] Processing binary operator: {:?}", op_token);
            let right = self.parse_binary(prec + 1);

            left = Expr::Binary {
                left: Box::new(left),
                op: match &op_token.value {
                    Some(lexer::TokenValue::Str(s)) => s.clone(),
                    _ => crate::fail(),
                },
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_prefix(&mut self) -> Expr {
        if let Some(token) = self.peek() {
            if let Some(op) = token_op(token) {
                if self.prefix_ops.contains(&op) {
                    self.advance();
                    let expr = self.parse_prefix();
                    return Expr::Unary {
                        oper: Box::new(expr),
                        op,
                    };
                }
            }
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            let token = match self.peek() {
                Some(t) => t,
                None => break,
            };

            match token._type {
                lexer::TokenType::BinaryOperator => {
                    if let Some(op) = token_op(token) {
                        if self.postfix_ops.contains(&op) {
                            self.advance();
                            expr = Expr::Unary {
                                oper: Box::new(expr),
                                op,
                            };
                            continue;
                        }
                    }
                    break;
                }

                lexer::TokenType::While => {
                    self.advance();
                    let body = self.parse_block();
                    expr = Expr::While {
                        cond: Box::new(expr),
                        then: Box::new(body),
                        else_then: Box::new(Expr::Block(vec![])),
                    };
                }

                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_primary with {}", self.tokens.get(self.pos).unwrap());
        match self.advance() {
            Some(lexer::Token {
                _type: lexer::TokenType::Const,
                value: Some(lexer::TokenValue::Str(s)),
            }) => {
                // eprintln!("[DEBUG] Parsed constant: {}", s);
                Expr::Const(s)
            },

            Some(lexer::Token {
                _type: lexer::TokenType::Variable,
                value: Some(lexer::TokenValue::Str(s)),
            }) => {
                // eprintln!("[DEBUG] Parsed variable: {}", s);
                Expr::Variable(s)
            },

            Some(lexer::Token {
                _type: lexer::TokenType::String,
                value: Some(lexer::TokenValue::Str(s)),
            }) => {
                match self.peek() {
                    Some(lexer::Token {_type: lexer::TokenType::LeftParen, ..}) => {
                        self.advance();
                        let mut args = Vec::new();
                        
                        while let Some(token) = self.peek() {
                            if token._type == lexer::TokenType::RightParen {
                                break;
                            }
                            if token._type == lexer::TokenType::Comma {
                                self.advance();
                                continue;
                            }
                            args.push(self.parse_expr());
                        }
                        
                        self.expect(&[lexer::TokenType::RightParen]);
                        Expr::Func {
                            name: s,
                            args,
                        }
                    },
                    Some(lexer::Token {_type: lexer::TokenType::Comma, .. }) => {
                        // eprintln!("[DEBUG] Decided string was part of array");
                        let mut array = Vec::<String>::new();
                        array.push(s);
                        while self.peek().unwrap()._type == lexer::TokenType::Comma {
                            self.expect(&[lexer::TokenType::Comma]);
                            match self.advance() {
                                Some(lexer:: Token {_type: lexer::TokenType::String, value: Some(lexer::TokenValue::Str(v))}) => {
                                    array.push(v);
                                },
                                _ => crate::fail()
                            }
                        }
                        Expr::Array(array)
                    },
                    _ => Expr::String(s)
                }
            },

            Some(lexer::Token {
                _type: lexer::TokenType::If,
                ..
            }) => {
                // eprintln!("[DEBUG] Parsing if expression");
                self.parse_if()
            },

            Some(lexer::Token {
                _type: lexer::TokenType::For,
                ..
            }) => {
                // eprintln!("[DEBUG] Parsing for expression");
                self.parse_for()
            },

            Some(lexer::Token {
                _type: lexer::TokenType::Try,
                ..
            }) => {
                // eprintln!("[DEBUG] Parsing try expression");
                self.parse_try()
            },

            Some(lexer::Token {
                _type: lexer::TokenType::Yield,
                ..
            }) => {
                // eprintln!("[DEBUG] Parsing yield expression");
                self.parse_yield()
            },

            Some(lexer::Token {
                _type: lexer::TokenType::Break,
                ..
            }) => Expr::Break(),

            Some(lexer::Token {
                _type: lexer::TokenType::LeftParen,
                ..
            }) => {
                let expr = self.parse_expr();
                self.expect(&[lexer::TokenType::RightParen]);
                expr
            }

            Some(lexer::Token {
                _type: lexer::TokenType::LeftCurly,
                ..
            }) => {
                // eprintln!("[DEBUG] Parsing block");
                self.parse_block()
            },

            Some(lexer::Token { _type: lexer::TokenType::Define, .. }) => {
                let var_token = self.expect(&[lexer::TokenType::Variable, lexer::TokenType::Const]);
                let var = match var_token.value {
                    Some(lexer::TokenValue::Str(s)) => s,
                    _ => crate::fail()
                };
                self.expect(&[lexer::TokenType::Assign]);

                let val = self.parse_expr();
                self.expect(&[lexer::TokenType::EndOfAssign]);
                Expr::Define {var, val: Box::new(val) }
            },

            Some(tok) => panic!(
                "unexpected token {:?} at pos {} with value {:?}",
                tok._type,
                self.pos - 1,
                tok.value
            ),
            None => crate::fail(),
        }
    }

    fn parse_block(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_block with {}", self.tokens.get(self.pos).unwrap());
        if let Some(token) = self.peek() {
            
            if token._type == lexer::TokenType::RightCurly {
                self.advance();
                return Expr::Block(vec![]);
            }
            
            if token._type != lexer::TokenType::Newline {
                let expr = self.parse_expr();
                self.expect(&[lexer::TokenType::RightCurly]);
                return Expr::Block(vec![expr]);
            }
        }
        self.expect(&[lexer::TokenType::Newline]);
        self.expect(&[lexer::TokenType::Indent]);

        let mut exprs = Vec::new();

        while let Some(token) = self.peek() {
            
            if token._type == lexer::TokenType::Dedent {
                
                break;
            }

            if token._type == lexer::TokenType::Newline {
                self.advance();
                continue;
            }

            exprs.push(self.parse_expr());
        }

        self.expect(&[lexer::TokenType::Dedent]);

        if matches!(self.peek(), Some(t) if t._type == lexer::TokenType::Newline) {
            self.advance();
        }

        self.expect(&[lexer::TokenType::RightCurly]);

        // eprintln!("[DEBUG] Block complete with {} expressions", exprs.len());
        Expr::Block(exprs)
    }

    fn parse_if(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_if with {}", self.tokens.get(self.pos).unwrap());
        let cond = self.parse_expr();
        self.expect(&[lexer::TokenType::LeftCurly]);
        let then_branch = self.parse_block();

        let else_branch = if let Some(token) = self.peek() {
            if token._type == lexer::TokenType::Else {
                self.advance();
                self.expect(&[lexer::TokenType::LeftCurly]);
                self.parse_block()
            } else {
                Expr::Block(vec![])
            }
        } else {
            Expr::Block(vec![])
        };

        Expr::If {
            cond: Box::new(cond),
            then: Box::new(then_branch),
            else_then: Box::new(else_branch),
        }
    }

    fn parse_for(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_for with {}", self.tokens.get(self.pos).unwrap());
        let token = self.tokens.get(self.pos - 1).unwrap();
        let iter = match &token.value {            Some(lexer::TokenValue::Num(n)) => *n,            Some(lexer::TokenValue::Str(s)) => {
                
                s.parse().unwrap_or_else(|_| crate::fail())
            }
            _ => crate::fail(),
        };

        let var = match self.expect(&[lexer::TokenType::Variable]).value {
            Some(lexer::TokenValue::Str(s)) => {
                
                s
            }
            _ => crate::fail(),
        };

        self.expect(&[lexer::TokenType::LeftCurly]);
        let body = self.parse_block();

        Expr::For {
            iter,
            var,
            then: Box::new(body),
            else_then: Box::new(Expr::Block(vec![])),
        }
    }

    fn parse_try(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_try with {}", self.tokens.get(self.pos).unwrap());
        let attempt = self.parse_expr();
        self.expect(&[lexer::TokenType::Catch]);
        let catch = self.parse_expr();

        Expr::Try {
            attempt: Box::new(attempt),
            catch: Box::new(catch),
        }
    }

    fn parse_yield(&mut self) -> Expr {
        // eprintln!("[DEBUG] parse_yield with {}", self.tokens.get(self.pos).unwrap());
        let expr = self.parse_expr();
        Expr::Yield(Box::new(expr))
    }
}

fn token_op(token: &lexer::Token) -> Option<String> {
    match (&token._type, &token.value) {
        (lexer::TokenType::BinaryOperator, Some(lexer::TokenValue::Str(s))) => Some(s.clone()),
        (lexer::TokenType::BinaryOperator, Some(lexer::TokenValue::Char(c))) => {
            Some(c.to_string())
        }
        _ => None,
    }
}

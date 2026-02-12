use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Define, Assign, EndOfAssign,
    If, Else, ElseIf,
    For, While, Break,
    Import, ImportAll,
    BinaryOperator,
    LeftParen, RightParen,
    Variable, Const, String, Comma,
    Indent, Dedent, Newline, LeftCurly, RightCurly,
    EOF, Continue, Yield, Try, Catch,
    Comment, BlockCommentStart, BlockCommentEnd
}

use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self)
    }
}

static KEYWORDS: LazyLock<Vec<(&'static str, TokenType)>> = LazyLock::new(|| {
    let mut v= vec![
        ("I would love to own a plot of land in the 1800s called", TokenType::Define),
        ("and lease it to", TokenType::Assign),
        ("sweet but stout", TokenType::ElseIf),
        ("American", TokenType::ImportAll),
        ("owners", TokenType::EndOfAssign),
        ("scammy", TokenType::Import),
        ("sweet", TokenType::If),
        ("stout", TokenType::Else),
        ("lolsie", TokenType::For),
        ("yarp'", TokenType::While),
        ("jump off the bandwagon", TokenType::Break),
        ("get back to work boy", TokenType::Continue),
        ("anywho", TokenType::Yield),
        ("sir, would there happen to be any extension work?", TokenType::Try),
        ("yay, homework!", TokenType::Catch),
        ("europe ->", TokenType::Comment),
        ("asia ->", TokenType::BlockCommentStart),
        ("<- asia", TokenType::BlockCommentEnd)
    ];

    v.sort_by_key(|(s, _): &(&str, TokenType)| std::cmp::Reverse(s.len()));

    return v;
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenValue {
    Char(char),
    Str(String),
    Num(usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<TokenValue>
}

impl Token {
    pub fn new(_type: TokenType, value: Option<TokenValue>) -> Self {
        Self {_type, value}
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::Char(c) => write!(f, "{}", c),
            TokenValue::Str(s) => write!(f, "{}", s),
            TokenValue::Num(n) => write!(f, "{}", n)
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "Type: {}, Value: {}", self._type, v),
            None => write!(f, "Type: {}, Value: None", self._type),
        }
    }
}

static OPERATORS: &[&str] = &["^", "*", "/", "+", "-", "<", ">", "=", "≥", "≤"];

fn consume_until_newline(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c == '\n' {
            break;
        }
        s.push(c);
        chars.next();
    }
    s
}

fn consume_until(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, end: &str) -> String {
    let mut s = String::new();

    loop {
        let mut it = chars.clone();
        let mut matched = true;

        for ec in end.chars() {
            match it.next() {
                Some(c) if c == ec => {}
                _ => {
                    matched = false;
                    break;
                }
            }
        }

        if matched {
            for _ in 0..end.len() {
                chars.next();
            }
            break;
        }

        match chars.next() {
            Some(c) => s.push(c),
            None => break,
        }
    }

    s
}

pub fn tokenize(src: &String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut indent_stack = vec![0];

    let mut chars = src.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                tokens.push(Token::new(TokenType::Newline, Some(TokenValue::Str("\\n".to_string()))));
                let mut indent = 0;
                while let Some(&next_c) = chars.peek() {
                    match next_c {
                        ' ' => { indent += 1; chars.next(); },
                        '\t' => { indent += 4; chars.next(); },
                        _ => break,
                    }
                }

                let current_indent = *indent_stack.last().unwrap();

                if indent > current_indent {
                    indent_stack.push(indent);
                    tokens.push(Token::new(TokenType::Indent, Some(TokenValue::Str("INDENT".to_string()))));
                } else if indent < current_indent {
                    while indent < *indent_stack.last().unwrap() {
                        indent_stack.pop();
                        tokens.push(Token::new(TokenType::Dedent, Some(TokenValue::Str("DEDENT".to_string()))));
                    }
                }

                continue;
            }
            ' ' | '\t' | '\r' => continue,

            '(' => tokens.push(Token::new(TokenType::LeftParen, Some(TokenValue::Char('(')))),
            ')' => tokens.push(Token::new(TokenType::RightParen, Some(TokenValue::Char(')')))),
            '{' => tokens.push(Token::new(TokenType::LeftCurly, Some(TokenValue::Char('{')))),
            '}' => tokens.push(Token::new(TokenType::RightCurly, Some(TokenValue::Char('}')))),
            ',' => tokens.push(Token::new(TokenType::Comma, Some(TokenValue::Char(',')))),
            op if OPERATORS.contains(&op.to_string().as_str()) => {
                let mut value = String::new();
                value.push(op.to_string().chars().next().unwrap());
                while let Some(&next_c) = chars.peek() {
                    if OPERATORS.contains(&next_c.to_string().as_str()) {
                        value.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::BinaryOperator, Some(TokenValue::Str(value))));
            },

            '$' => {
                let mut value = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_alphanumeric() || next_c == '_' {
                        value.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::Const, Some(TokenValue::Str(value))));
            }

            '£' | '€' => {
                let mut value = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_alphanumeric() || next_c == '_' {
                        value.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::Variable, Some(TokenValue::Str(value))));
            }
            _ => {
                let mut matched: Option<TokenType> = None;
                let mut keyword = "";
                let mut matched_len = 0;

                for (kw, ty) in KEYWORDS.iter() {
                    if c == kw.chars().next().unwrap() {
                        let mut it = chars.clone();
                        let mut ok = true;

                        for expected in kw.chars().skip(1) {
                            match it.next() {
                                Some(actual) if actual == expected => {}
                                _ => {
                                    ok = false;
                                    break;
                                }
                            }
                        }

                        if ok {
                            matched = Some(ty.clone());
                            keyword = kw;
                            matched_len = kw.len() - 1;
                            break;
                        }
                    }
                };
                
                if let Some(token_type) = matched {
                    for _ in 0..matched_len {
                        chars.next();
                    }

                    match token_type {
                        TokenType::Comment => {
                            let text = consume_until_newline(&mut chars);
                            tokens.push(Token::new(
                                TokenType::Comment,
                                Some(TokenValue::Str(text)),
                            ));
                        }

                        TokenType::BlockCommentStart => {
                            let text = consume_until(&mut chars, "<- asia");
                            tokens.push(Token::new(
                                TokenType::BlockCommentStart,
                                Some(TokenValue::Str(text)),
                            ));
                            tokens.push(Token::new(TokenType::BlockCommentEnd, None));
                        }

                        TokenType::For => {
                            let mut it = 0;
                            while let Some(&next_c) = chars.peek() {
                                if next_c != 's' {
                                    break;
                                }
                                it += 1;
                                chars.next();
                            }
                            tokens.push(Token::new(TokenType::For, Some(TokenValue::Num(it))));
                        }

                        _ => {
                            tokens.push(Token::new(
                                token_type,
                                Some(TokenValue::Str(keyword.to_string())),
                            ));
                        }
                    }
                } else {
                    let mut value = String::new();
                    value.push(c);

                    while let Some(&next_c) = chars.peek() {
                        if (next_c.is_whitespace() || "(){},+-*/$£".contains(next_c)) && c != '\\' {
                            break;
                        }
                        value.push(next_c);
                        chars.next();
                    }

                    tokens.push(Token::new(TokenType::String, Some(TokenValue::Str(value))));
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, Some(TokenValue::Str("EOF".to_string()))));

    return tokens
}

use std::collections::HashMap;

use crate::lexer::{self, TokenType, TokenValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorKind {
    Prefix,
    Postfix,
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorDef {
    pub op: String,
    pub func: Option<String>,
    pub precedence: usize,
    pub kind: OperatorKind,
}


fn default_precedence_map() -> HashMap<String, usize> {
    let mut precedence_map: HashMap<String, usize> = HashMap::new();

    precedence_map.insert("^^".to_string(), 4);
    precedence_map.insert("^".to_string(), 3);
    precedence_map.insert("*".to_string(), 2);
    precedence_map.insert("/".to_string(), 2);
    precedence_map.insert("+".to_string(), 1);
    precedence_map.insert("-".to_string(), 1);
    precedence_map.insert("<".to_string(), 0);
    precedence_map.insert(">".to_string(), 0);
    precedence_map.insert("=".to_string(), 0);
    precedence_map.insert("≥".to_string(), 0);
    precedence_map.insert("≤".to_string(), 0);

    precedence_map
}




pub fn run(
    tokens: Vec<lexer::Token>,
) -> (Vec<lexer::Token>, HashMap<String, usize>, Vec<OperatorDef>) {
    let mut filtered = Vec::with_capacity(tokens.len());
    let mut precedence_map = default_precedence_map();
    let mut defs = Vec::<OperatorDef>::new();

    let mut i = 0;
    while i < tokens.len() {
        if let Some(op_def) = maybe_parse_op_def(&tokens, i) {
            
            precedence_map.insert(op_def.op.clone(), op_def.precedence);
            defs.push(op_def);

            
            i = skip_until_end_of_assign(&tokens, i + 1);
            continue;
        }

        filtered.push(tokens[i].clone());
        i += 1;
    }

    (filtered, precedence_map, defs)
}

fn skip_until_end_of_assign(tokens: &[lexer::Token], mut i: usize) -> usize {
    while i < tokens.len() {
        if tokens[i]._type == TokenType::EndOfAssign {
            return i + 1;
        }
        i += 1;
    }
    i
}

fn maybe_parse_op_def(tokens: &[lexer::Token], i: usize) -> Option<OperatorDef> {
    
    let t = tokens.get(i)?;
    if t._type != TokenType::Define {
        return None;
    }

    let op_token = tokens.get(i + 1)?;
    let assign_token = tokens.get(i + 2)?;
    let op_is_name = matches!(op_token._type, TokenType::BinaryOperator);
    if !op_is_name || assign_token._type != TokenType::Assign {
        return None;
    }

    let mut cursor = i + 3;

    
    let (func, precedence, kind, _consumed) =
        if matches!(tokens.get(cursor), Some(tok) if tok._type == TokenType::LeftCurly) {
            cursor += 1;
            let func = match tokens.get(cursor) {
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) => Some(s.clone()),
                _ => None,
            };
            cursor += 1; 

            if matches!(tokens.get(cursor), Some(tok) if tok._type == TokenType::Comma) {
                cursor += 1;
            }

            let precedence = match tokens.get(cursor) {
                Some(lexer::Token {
                    value: Some(TokenValue::Num(n)),
                    ..
                }) => *n,
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) => s.parse().unwrap_or(0),
                _ => 0,
            };
            cursor += 1;

            if matches!(tokens.get(cursor), Some(tok) if tok._type == TokenType::Comma) {
                cursor += 1;
            }

            let kind = match tokens.get(cursor) {
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) if s.eq_ignore_ascii_case("prefix") => OperatorKind::Prefix,
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) if s.eq_ignore_ascii_case("postfix") => OperatorKind::Postfix,
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) if s.eq_ignore_ascii_case("binary") => OperatorKind::Binary,
                Some(lexer::Token {
                    value: Some(TokenValue::Str(s)),
                    ..
                }) if s.eq_ignore_ascii_case("unary") => OperatorKind::Postfix,
                _ => OperatorKind::Binary,
            };
            cursor += 1; 

            if matches!(tokens.get(cursor), Some(tok) if tok._type == TokenType::RightCurly) {
                cursor += 1;
            }

            (func, precedence, kind, cursor - (i + 3))
        } else {
            (None, 0, OperatorKind::Binary, 0)
        };

    
    while cursor < tokens.len() {
        if tokens[cursor]._type == TokenType::EndOfAssign {
            let op = match &op_token.value {
                Some(TokenValue::Str(s)) => s.clone(),
                _ => return None,
            };

            return Some(OperatorDef {
                op,
                func,
                precedence,
                kind,
            });
        }
        cursor += 1;
    }

    
    None
}

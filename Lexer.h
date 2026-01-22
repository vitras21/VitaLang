//
// Created by Naoise MCGINNITY on 17/01/2026.
//

#ifndef VITALANG_LEXER_H
#define VITALANG_LEXER_H
#include <string>
#include <vector>
#include <unordered_map>
#include <ostream>

enum class TokenType {
    Define, Assign, EndOfAssign,
    If, Else, ElseIf,
    True, False, None,
    For, While, Break,
    Import, ImportAll,
    BinaryOperator,
    LeftParen, RightParen,
    Identifier,
    Number,
    Unknown,
    Indent, Dedent, Newline, LeftCurly, RightCurly
};

inline std::string to_string(TokenType t) {
    switch (t) {
        case TokenType::Define:         return "Define";
        case TokenType::Assign:         return "Assign";
        case TokenType::EndOfAssign:    return "EndOfAssign";
        case TokenType::If:             return "If";
        case TokenType::Else:           return "Else";
        case TokenType::ElseIf:         return "ElseIf";
        case TokenType::True:           return "True";
        case TokenType::False:          return "False";
        case TokenType::None:           return "None";
        case TokenType::For:            return "For";
        case TokenType::While:          return "While";
        case TokenType::Break:          return "Break";
        case TokenType::Import:         return "Import";
        case TokenType::ImportAll:      return "ImportAll";
        case TokenType::BinaryOperator: return "BinaryOperator";
        case TokenType::LeftParen:      return "LeftParen";
        case TokenType::RightParen:     return "RightParen";
        case TokenType::Identifier:     return "Identifier";
        case TokenType::Number:         return "Number";
        case TokenType::Unknown:        return "Unknown";
        case TokenType::Indent:         return "Indent";
        case TokenType::Dedent:         return "Dedent";
        case TokenType::Newline:        return "Newline";
        case TokenType::LeftCurly:      return "LeftCurly";
        case TokenType::RightCurly:     return "RightCurly";
    }
    return "Invalid";
}

class Token {
public:
    std::string value;
    TokenType type;

    Token(std::string v, TokenType t);
};

inline std::ostream& operator<<(std::ostream& os, const Token& token) {
    os << "Token(" << token.value << ", " << static_cast<int>(token.type) << ")";
    return os;
}

extern const std::unordered_map<std::string, TokenType> keywords;

bool isspace(char c);
bool isdigit(char c);
bool isalpha(char c);

std::vector<Token> tokenize(const std::string& src);

#endif // VITALANG_LEXER_H
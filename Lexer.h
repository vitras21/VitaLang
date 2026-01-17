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
    Unknown
};

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
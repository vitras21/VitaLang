//
// Created by Naoise MCGINNITY on 16/01/2026.
//

#include "Lexer.h"
#include <unordered_map>
#include <string>
#include <vector>
#include <cstring>

const std::unordered_map<std::string, TokenType> keywords = {
    {"I would love to own a plot of land in the 1800s called", TokenType::Define},
    {"and lease it to", TokenType::Assign},
    {"owners", TokenType::EndOfAssign},
    {"not particularly", TokenType::True},
    {"not not particularly", TokenType::False},
    {"context", TokenType::None},
    {"scammy", TokenType::Import},
    {"American", TokenType::ImportAll}
};

bool isspace(char c) {return (strchr(" \t\n", c));};
bool isdigit(char c) {return (strchr("0123456789", c));};
bool isalpha(char c) {return (std::isalpha(c));};

Token::Token(std::string value, TokenType type)
    : value(std::move(value)), type(type) {}

std::vector<Token> tokenize(const std::string& src) {
    std::vector<Token> tokens;
    size_t i = 0;

    while (i < src.length()) {

        if (isspace(src[i])) {
            i++;
            continue;
        }

        if (src[i] == '(') {
            tokens.emplace_back("(", TokenType::LeftParen);
            i++;
            continue;
        }

        if (strchr("+-*/", src[i])) {
            tokens.emplace_back(std::string(1, src[i]), TokenType::BinaryOperator);
            i++;
            continue;
        }

        bool matched = false;
        for (auto& [key, type] : keywords) {
            if (src.compare(i, key.length(), key) == 0) {
                tokens.emplace_back(key, type);
                i += key.length();
                matched = true;
                break;
            }
        }

        if (matched) continue;

        if (isalpha(src[i])) {
            size_t start = i;
            while (isalnum(src[i])) i++;
            tokens.emplace_back(
                src.substr(start, i - start),
                TokenType::Identifier
            );
            continue;
        }

        if (isdigit(src[i])) {
            size_t start = i;
            while (isdigit(src[i])) i++;
            tokens.emplace_back(
                src.substr(start, i - start),
                TokenType::Number
            );
            continue;
        }

        tokens.emplace_back(std::string(1, src[i]), TokenType::Unknown);
        i++;
    }

    return tokens;
};
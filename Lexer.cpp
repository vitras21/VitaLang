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
    {"American", TokenType::ImportAll},
    {"sweet", TokenType::If},
    {"stout", TokenType::Else},
    {"sweet but stout", TokenType::ElseIf},
    {"lolsie", TokenType::For}
};

bool is_digit(const char c) {
    return c >= '0' && c <= '9';
}

bool is_alpha(const char c) {
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
           c == '_';
}

Token::Token(std::string value, TokenType type)
    : value(std::move(value)), type(type) {}

std::vector<Token> tokenize(const std::string& src) {
    std::vector<Token> tokens;
    size_t i = 0;
    std::vector<int> indentStack;
    indentStack.push_back(0);

    while (i < src.length()) {

        if (src[i] == '\n') {
            tokens.emplace_back("\\n", TokenType::Newline);
            i++;

            int indent = 0;
            while (i < src.length()) {
                if (src[i] == ' ') {
                    indent++;
                } else if (src[i] == '\t') {
                    indent += 4;
                } else {
                    break;
                }
                i++;
            }

            int currentIndent = indentStack.back();

            if (indent > currentIndent) {
                indentStack.push_back(indent);
                tokens.emplace_back("INDENT", TokenType::Indent);
            }
            else if (indent < currentIndent) {
                while (indent < indentStack.back()) {
                    indentStack.pop_back();
                    tokens.emplace_back("DEDENT", TokenType::Dedent);
                }
            }

            continue;
        }

        if (src[i] == ' ' || src[i] == '\t' || src[i] == '\r') {
            i++;
            continue;
        }

        if (src[i] == '(') {
            tokens.emplace_back("(", TokenType::LeftParen);
            i++;
            continue;
        }

        if (src[i] == ')') {
            tokens.emplace_back(")", TokenType::RightParen);
            i++;
            continue;
        }

        if (src[i] == '{') {
            tokens.emplace_back("{", TokenType::LeftCurly);
            i++;
            continue;
        }

        if (src[i] == '}') {
            tokens.emplace_back("}", TokenType::RightCurly);
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
            if (src.compare(i, key.length(), key) == 0 && (i + key.length() == src.length() || (!is_alpha(src[i + key.length()] && type != TokenType::For)))) {
                i += key.length();
                if (type == TokenType::For) {
                    int n = 0;
                    while (i < src.length() && src[i] == std::string("s")[0]) {n++; i++;}
                    tokens.emplace_back(std::to_string(n), type);
                } else {
                    tokens.emplace_back(key, type);
                }
                matched = true;
                break;
            }
        }

        if (matched) continue;

        if (is_alpha(src[i])) {
            size_t start = i;
            while (i < src.length() && isalnum(src[i])) i++;
            tokens.emplace_back(
                src.substr(start, i - start),
                TokenType::Identifier
            );
            continue;
        }

        if (is_digit(src[i])) {
            size_t start = i;
            while (i < src.length() && is_digit(src[i])) i++;
            tokens.emplace_back(
                src.substr(start, i - start),
                TokenType::Number
            );
            continue;
        }

        tokens.emplace_back(std::string(1, src[i]), TokenType::Unknown);
        i++;
    }

    while (indentStack.size() > 1) {
        indentStack.pop_back();
        tokens.emplace_back("DEDENT", TokenType::Dedent);
    }

    return tokens;
};
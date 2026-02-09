//
// Created by Naoise MCGINNITY on 18/01/2026.
//

#include "Parser.h"
#include "Main.h"

#include <memory>

template<typename... Types>
void expect(const Token& token, Types... types) {
    if (!((token.type == types) || ...)) {
        throw Context("Unexpected token of type" + to_string(token.type) + ": " + token.value);
    }
}

std::unique_ptr<Expression> parsePrimary(const std::vector<Token>& tokens, size_t& i) {
    if (tokens[i].type == TokenType::Number ||
        tokens[i].type == TokenType::Variable || tokens[i].type == TokenType::String || tokens[i].type == TokenType::Const) {
        return std::make_unique<LiteralExpression>(tokens[i++]);
        }

    throw Context("Unidentifiable Operator");
}

int precedence(const Token& token) {
    if (token.value == "*" || token.value == "/") {return 2;}
    if (token.value == "+" || token.value == "-") {return 1;}
    return -1;
}

std::unique_ptr<Expression> parseExpression(
    const std::vector<Token>& tokens,
    size_t& i,
    int minPrec = 0
) {
    auto left = parsePrimary(tokens, i);

    while (i < tokens.size() &&
           tokens[i].type == TokenType::BinaryOperator &&
           precedence(tokens[i]) >= minPrec) {

        Token op = tokens[i++];
        int prec = precedence(op);

        auto right = parseExpression(tokens, i, prec + 1);

        left = std::make_unique<BinaryExpression>(
            std::move(left),
            op,
            std::move(right)
        );
           }

    return left;
};

std::unique_ptr<Expression> parseArray(const std::vector<Token>& tokens, size_t& i) {
    std::vector<std::unique_ptr<Expression>> elements;
    elements.push_back(parseExpression(tokens, i));

    while (i < tokens.size() && tokens[i].type == TokenType::Comma) {
        i++;
        elements.push_back(parseExpression(tokens, i));
    }

    return std::make_unique<ArrayExpression>(std::move(elements));
}

std::unique_ptr<ForStatement> parseFor(const std::vector<Token>& tokens, size_t& i) {
    expect(tokens[i], TokenType::For);
    const size_t n = std::stoul(tokens[i].value);
    i++;


    expect(tokens[i], TokenType::Variable);
    Token id = tokens[i];
    i++;

    auto body = parseBlock(tokens);

    return std::make_unique<ForStatement>(n, std::move(id), std::move(body));
}

std::unique_ptr<IfStatement> parseIf(const std::vector<Token>& tokens, size_t& i) {
    expect(tokens[i], TokenType::If);
    i++;

    std::vector<Token> tokenExpr;
    while (tokens[i].type != TokenType::LeftCurly) {
        tokenExpr.push_back(tokens[i]);
        i++;
    }

    size_t indexExpr = 0;
    auto condition = parseExpression(tokenExpr, indexExpr, 0);
    auto body = parseBlock(tokens);

    return std::make_unique<IfStatement>(
        std::move(condition),
        std::move(body)
    );
}

std::unique_ptr<WhileStatement> parseWhile(const std::vector<Token>& tokens, size_t& i) {
    expect(tokens[i], TokenType::LeftParen);
    i++;

    std::vector<Token> tokenExpr;
    while (tokens[i].type != TokenType::RightParen) {
        tokenExpr.push_back(tokens[i]);
        i++;
    }

    size_t indexExpr = 0;
    auto condition = parseExpression(tokenExpr, indexExpr, 0);

    expect(tokens[i], TokenType::RightParen);
    i++;

    expect(tokens[i], TokenType::While);
    i++;

    auto body = parseBlock(tokens);

    return std::make_unique<WhileStatement>(
        std::move(condition),
        std::move(body)
    );
}

std::unique_ptr<Assignment> parseAssignment(const std::vector<Token>& tokens, size_t& i) {
    expect(tokens[i], TokenType::Define);
    i++;

    expect(tokens[i], TokenType::Const, TokenType::Variable);
    Token id = tokens[i];
    i++;

    expect(tokens[i], TokenType::Assign);
    i++;

    std::vector<Token> tokenExpr;
    while (tokens[i].type != TokenType::EndOfAssign) {
        tokenExpr.push_back(tokens[i]);
        i++;
    }

    size_t indexExpr = 0;
    auto expr = parseExpression(tokenExpr, indexExpr, 0);

    expect(tokens[i], TokenType::EndOfAssign);
    i++;

    return std::make_unique<Assignment>(
        std::move(id),
        std::move(expr)
    );
}

std::unique_ptr<Statement> parseStatement(const std::vector<Token>& tokens, size_t& i) {

    if (tokens[i].type == TokenType::For) {
        return parseFor(tokens, i);
    }

    if (tokens[i].type == TokenType::If) {
        return parseIf(tokens, i);
    }

    if (tokens[i].type == TokenType::LeftParen) {
        size_t j = i;
        while (tokens[j].type != TokenType::RightParen) {
            j++;
        }
        if (tokens[j + 1].type == TokenType::While) {
            return parseWhile(tokens, i);
        }
    }

    if (tokens[i].type == TokenType::Define) {
        return parseAssignment(tokens, i);
    }

    throw Context("Unrecognized token of type: " + to_string(tokens[i].type) + "with value: " + tokens[i].value);
}

std::vector<std::unique_ptr<ASTNode>> parseBlock(const std::vector<Token>& tokens, size_t& i) {
    expect(tokens[i], TokenType::LeftCurly);
    i++;

    expect(tokens[i], TokenType::Newline);
    i++;

    expect(tokens[i], TokenType::Indent);
    i++;

    std::vector<std::unique_ptr<ASTNode>> body;

    while (tokens[i].type != TokenType::Dedent) {
        if (tokens[i].type == TokenType::Newline) {
            i++;
            continue;
        }
        body.push_back(parseStatement(tokens));
    }

    expect(tokens[i], TokenType::Dedent);
    i++;

    expect(tokens[i], TokenType::RightCurly);
    i++;

    return body;
}

std::vector<std::unique_ptr<ASTNode>> parse(const std::vector<Token>& tokens) {
    size_t i = 0;
    std::vector<std::unique_ptr<ASTNode>> program;

    while (i < tokens.size()) {
        if (tokens[i].type == TokenType::LeftCurly) {
            auto block = parseBlock(tokens);
            for (auto& node : block) {
                program.push_back(std::move(node));
            }
        } else if (tokens[i].type == TokenType::Newline) {
            i++;
        } else {
            program.push_back(parseStatement(tokens));
        }
    }

    return program;
}
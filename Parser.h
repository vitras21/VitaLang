//
// Created by Naoise MCGINNITY on 18/01/2026.
//

#ifndef VITALANG_PARSER_H
#define VITALANG_PARSER_H

#include <memory>
#include <utility>
#include <vector>

#include "Lexer.h"

// =====================
// Forward declarations
// =====================
struct ASTNode;
struct Expression;
struct Statement;

std::unique_ptr<Statement> parseStatement(const std::vector<Token>& tokens);
std::vector<std::unique_ptr<ASTNode>> parseBlock(const std::vector<Token>& tokens);
std::vector<std::unique_ptr<ASTNode>> parse(const std::vector<Token>& tokens);

struct ASTNode {
    virtual ~ASTNode() = default;
};

struct Expression : ASTNode {
    virtual ~Expression() = default;
};

struct Statement : ASTNode {
    virtual ~Statement() = default;
};

struct ArrayExpression final : Expression {
    std::vector<std::unique_ptr<Expression>> elements;

    explicit ArrayExpression(std::vector<std::unique_ptr<Expression>> elements) : elements(std::move(elements)) {}
};

struct LiteralExpression final : Expression {
    Token value;

    explicit LiteralExpression(Token value)
        : value(std::move(value)) {}
};

struct BinaryExpression final : Expression {
    std::unique_ptr<Expression> left;
    Token op;
    std::unique_ptr<Expression> right;

    BinaryExpression(
        std::unique_ptr<Expression> left,
        Token op,
        std::unique_ptr<Expression> right
    )
        : left(std::move(left)),
          op(std::move(op)),
          right(std::move(right)) {}
};

struct IfStatement final : Statement {
    std::unique_ptr<Expression> condition;
    std::vector<std::unique_ptr<ASTNode>> body;

    IfStatement(
        std::unique_ptr<Expression> condition,
        std::vector<std::unique_ptr<ASTNode>> body
    )
        : condition(std::move(condition)),
          body(std::move(body)) {}
};

struct Assignment final : Statement {
    Token id;
    std::unique_ptr<Expression> expr;

    Assignment(
        Token id,
        std::unique_ptr<Expression> expr
    )
        : id(std::move(id)),
          expr(std::move(expr)) {}
};

struct WhileStatement final : Statement {
    std::unique_ptr<Expression> condition;
    std::vector<std::unique_ptr<ASTNode>> body;

    WhileStatement(
        std::unique_ptr<Expression> condition,
        std::vector<std::unique_ptr<ASTNode>> body
    )
        : condition(std::move(condition)),
          body(std::move(body)) {}
};

struct ForStatement final : Statement {
    size_t n;
    Token id;
    std::vector<std::unique_ptr<ASTNode>> body;

    ForStatement(
        size_t n,
        Token id,
        std::vector<std::unique_ptr<ASTNode>> body
    )
        : n(n),
          id(std::move(id)),
          body(std::move(body)) {}
};

#endif // VITALANG_PARSER_H
//
// Created by Naoise MCGINNITY on 17/01/2026.
//

#include "Main.h"

#include <iostream>

#include "Lexer.h"
#include "Parser.h"

std::string readFile(const std::string& filename) {
    std::ifstream file(filename);
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
};

void onTerminate() {
    std::exception_ptr eptr = std::current_exception();

    if (eptr) {
        try {
            std::rethrow_exception(eptr);
        } catch (const std::exception& e) {
            std::cerr << e.what() << std::endl;
        }
    }

    std::abort();
};

int main(int argc, char** argv) {
    std::set_terminate(onTerminate);
    const std::string script = readFile(argv[1]);

    std::vector<Token> tokens = tokenize(script);

    for (const auto& token : tokens) {
        std::cout << token << std::endl;
    }

//    std::vector<ASTNode> AST = parse(tokens);

    return 0;
};
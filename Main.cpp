//
// Created by Naoise MCGINNITY on 17/01/2026.
//

#include "Main.h"

#include <iostream>

#include "Lexer.h"

std::string readFile(const std::string& filename) {
    std::ifstream file(filename);
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

int main(int argc, char** argv) {
    const std::string script = readFile(argv[1]);

    std::vector<Token> tokens = tokenize(script);

    for (const auto& token : tokens) {
        std::cout << token << std::endl;
    }

    return 0;
};
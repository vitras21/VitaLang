//
// Created by Naoise MCGINNITY on 17/01/2026.
//

#ifndef VITALANG_MAIN_H
#define VITALANG_MAIN_H
#include <fstream>
#include <sstream>
#include <string>

std::string readFile(const std::string& filename);

struct Context : std::exception {
    std::string message;

    Context(const std::string& message) : message("There is context. Definitely.") {}

    const char* what() const noexcept override { return message.c_str(); }
};


#endif //VITALANG_MAIN_H
#include <stddef.h>
#include <iostream>
#include <stdexcept>
#include <stdlib.h>
#include <string>
#include <unordered_map>
#include <vector>

#include "eva.hpp"

int main() {
    eva parser("test.eva");

    try {
        auto project_name = parser.get<std::string>("project", "name");
        std::cout << "Running " << project_name << " project" << std::endl;
    } catch(std::runtime_error e) {}

    std::string name;
    try {
        name = parser.get<std::string>("dev", "name");
        std::cout << "created by: " << name << std::endl;
    } catch(std::runtime_error e) {}

    try {
        auto dev_messages = parser.get<eva::list>("dev", "messages");
        int index = 0;
        std::cout << name << " said: " << dev_messages.operator[]<std::string>(index) << std::endl;
    } catch(std::runtime_error e) {}
    return 0;
}

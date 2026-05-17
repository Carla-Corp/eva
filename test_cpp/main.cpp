#include <stddef.h>
#include <iostream>
#include <stdlib.h>
#include <string>
#include <unordered_map>
#include <vector>

#include "eva.hpp"

int main() {
    eva parser("test.eva");
    auto value = parser.get<eva::map>("data", "dev");
    auto inner_value = value.operator[]<eva::list>("jobs");
    std::cout << inner_value.operator[]<std::string>(0) << std::endl;
    return 0;
}

#include <stddef.h>
#include <iostream>
#include <stdlib.h>
#include <string>

#include "eva.hpp"

int main() {
    eva parser("test.eva");

    {   auto [exist, project_name] = parser.get<std::string>("project", "name");
        if(! exist ) return 1;
        std::cout << "Running " << project_name << " project" << std::endl;
    }

    auto [exist, name] = parser.get<std::string>("dev", "name");
    if(! exist ) return 1;
    std::cout << "created by: " << name << std::endl;

    {   auto [exist, dev_messages] = parser.get<eva::list>("dev", "messages");
        if(! exist ) return 1;

        int index = 0;
        std::cout << name << " said: " << eva::data(dev_messages.operator[]<std::string>(index)) << std::endl;
    }

    return 0;
}

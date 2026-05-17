#include <stddef.h>

typedef struct EvaParser {
    size_t status;
    void const *parser;
} EvaParser;

EvaParser *make_parser(const char *path);
void print_value(EvaParser *parser, const char *namespace, const char *field);

int main() {
    EvaParser *parser = make_parser("test.eva");
    print_value(parser, "dev", "msg");
    return 0;

}

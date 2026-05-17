#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct EvaParser {
    size_t status;
    void const *parser;
} EvaParser;

typedef enum {
    eva_string,
    eva_number,
    eva_bool,
    eva_nil,
} EvaValueTag;

typedef struct EvaValue {
    EvaValueTag tag;
    union {
        char *string;
        double number;
        int boolean;
    } data;
} EvaValue;

EvaParser *eva_make_parser(const char *path);
EvaValue eva_get_value_from_namespace(EvaParser *parser, const char *namespace, const char *name);

int main() {
    EvaParser *parser = eva_make_parser("test.eva");
    EvaValue value = eva_get_value_from_namespace(parser, "dev", "msg");
    if( value.tag == eva_string && value.data.string ) {
        printf("%s\n", value.data.string);
        free(value.data.string);
    }


    return 0;

}

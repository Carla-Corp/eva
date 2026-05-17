#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "eva.h"

int main() {
    EvaParser *parser = eva_make("test.eva");
    EvaValue value = eva_get(parser, "data", "dev");

    int len = eva_maplen(value);
    printf(" sizeof map: %d\n", len);
    if( len > 0 && eva_mapexist(value, "msg") ) {
        EvaValue result = eva_mapget(value, "msg");
        if( result.tag == eva_string ) {
            printf(" result: %s\n", result.data.string);
        }
    }

    return 0;

}

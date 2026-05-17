#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "eva.h"

int main() {
    EvaParser *parser = eva_make("test.eva");
    EvaValue project_name = eva_get(parser, "project", "name");

    if( project_name.tag == eva_string ) {
        printf("Runnning %s project\n", project_name.data.string);
    }

    char *name;
    EvaValue dev_name = eva_get(parser, "dev", "name");
    if( dev_name.tag == eva_string ) {
        name = dev_name.data.string;
        printf("created by: %s\n", name);
    }

    EvaValue dev_messages = eva_get(parser, "dev", "messages");
    if( dev_messages.tag == eva_list ) {
        int index = 0;
        printf("%s said: %s\n", name, eva_listget(dev_messages, index).data.string);
    }

    return 0;
}

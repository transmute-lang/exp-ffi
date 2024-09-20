#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

#include "../gen/exp-ffi.h"

int main() {
    char *data = query();

    printf("%s\n", data);

    query_free(data);

    return EXIT_SUCCESS;
}
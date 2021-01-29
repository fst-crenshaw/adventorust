// foo.c
#include <stdio.h>

#include "myrust.h"

int main() {
    noop();
    printf("hello\n");
    printf("1+2=%ld\n", add(1, 2));

    return 0;
}

#include <stdio.h>
#include "repl.h"

int main() {
    intro();
    char choice = marker_choice();

    printf("%s", choice);

    return 0;
}

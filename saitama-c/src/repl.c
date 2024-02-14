#include <stdio.h>


void intro() {
    printf("welcome to tic tac toe!");
}

char marker_choice() {
    char choice;
    scanf("%c", &choice);
    return choice;
}

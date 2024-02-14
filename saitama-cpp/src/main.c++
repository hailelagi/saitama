#include <iostream>
#include <string>
#include <sstream>

#include "./repl.h"

using namespace std;

int main() {
    intro();
    int marker = 0;

    cin >> marker;

    cout << "This " << marker;
    return 0;
}

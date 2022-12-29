#include <stdio.h>
#include "autotyper.h"

int main(int argc, char *argv[]) {
    printf("testing\n");

    AutoTyper at = create();
    set_file(&at, "../sample.yaml");
    set_start_delay(&at, 9);
    set_input_delay(&at, 270);
    print(&at);

    configure(&at);
    if (has_next(&at)) {
        printf("has next is true\n");

        next(&at);
    } else {
        printf("has next is false\n");
    }

    printf("\nfinished testing\n");

    return 0;
}

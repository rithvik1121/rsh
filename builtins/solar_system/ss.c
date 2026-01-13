#include <stdio.h>
#include <curl/curl.h>
#include <ncurses.h>


int main() {
    printf("Hello World\n");

    initscr();

    refresh();
    endwin();
}

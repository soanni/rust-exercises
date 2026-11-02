#include <stdio.h>

int main(void) {
  int i = 10;

  do {
    printf("%d\n", --i);
  } while (i > 0);
}

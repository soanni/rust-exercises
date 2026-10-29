#include <stdio.h>

int main(void) {
  int a = 1, b, c;
  // either -1 or 1
  // // undefined
  c = (b = a + 1) - (a = 3);
  printf("c=%d\n", c);
}

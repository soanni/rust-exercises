#include <stdio.h>

int main(void) {
  int i = 2, j;
  // either 4 or 6
  // undefined
  j = i * i++;
  printf("j=%d\n", j);
}

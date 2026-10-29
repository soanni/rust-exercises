#include <stdio.h>

int main(void) {
  int i, j, k;
  i = 1;
  k = 1 + (j = i);
  printf("%d\t%d\t%d\n", i, j, k);
  return 0;
}

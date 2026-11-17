#include <stdio.h>

int main() {
  int i, j, *p, *q = &j;
  i = 1;
  p = &i;
  *p = 2;
  *q = *p; /* 2 is assigned to j, q doesnt point on i */

  printf("*q = %d\n", *q);
  return 0;
}

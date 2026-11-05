#include <stdio.h>

int main(void) {
  int a[10] = {1, 2, 3}, b[10], c[10] = {0}, i;

  int d[] = {4, 3, 2, 1};

  int e[3] = {5, 5, 5, 5}; /* WRONG */

  int f[2] = {}; /* WRONG */

  for (i = 0; i < 10; i++) {
    printf("%d ", a[i]);
  }
  printf("\n");

  for (i = 0; i < 10; i++) {
    printf("%d ", b[i]);
  }
  printf("\n");

  for (i = 0; i < 10; i++) {
    printf("%d ", c[i]);
  }
  printf("\n");

  return 0;
}

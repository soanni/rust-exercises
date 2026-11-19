#include <stdio.h>

#define N 10

int main(void) {
  int a[N] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, sum = 0, *p;

  for (p = &a[0]; p < &a[N]; p++) {
    sum += *p;
  }

  printf("sum = %d\n", sum);
}

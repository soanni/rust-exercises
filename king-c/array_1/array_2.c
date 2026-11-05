#include <stdio.h>

#define N 10

int main(void) {
  int a[N], i = 0;

  while (i < N) {
    a[i++] = 0;
  }

  for (int j = 0; j < N; j++) {
    printf("%d ", a[j]);
  }

  printf("\n");
}

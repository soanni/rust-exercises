#include <stdio.h>

#define N 3

int main(void) {
  int a[N] = {1, 1, 1};
  int b[N] = {2, 2, 2};
  int i = 0;

  while (i < N) {
    a[i] += b[i++];
  }

  for (int j = 0; j < N; j++) {
    printf("%d ", a[j]);
  }

  printf("\n");
}

#include <stdio.h>

#define N 10

int main(void) {
  int a[N], i;

  printf("Enter a sequence of numbers: ");

  for (i = 0; i < N; i++) {
    scanf("%d", &a[i]);
  }

  printf("Reverse is: ");

  /*while (i >= 0) {
        printf("%d ", a[--i]);
  }
  */

  for (int j = 0; j < N; j++) {
    printf("%d ", a[N - 1 - j]);
  }

  printf("\n");

  return 0;
}

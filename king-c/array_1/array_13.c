#include <stdio.h>

#define N 3

int arr_sum(int[][N], int);

/* note that number of columns MUST be provided */
int main(void) {
  int b[][N] = {{1, 2, 3}, {4, 5, 6}};

  printf("arr sum is %d\n", arr_sum(b, 2));

  return 0;
}

int arr_sum(int a[][N], int n) { /* n - rows count */
  int sum = 0;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < N; j++) {
      sum += a[i][j];
    }
  }

  return sum;
}

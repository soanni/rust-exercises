#include <stdio.h>

#define N 5

int arr_sum(int[], int);

int main(void) {
  int b[N] = {1, 1, 1, 1, 1};

  printf("arr sum is %d\n", arr_sum(b, N + 10)); /* UB */

  return 0;
}

int arr_sum(int a[], int n) {
  int sum = 0;
  for (int i = 0; i < n; i++) {
    sum += a[i];
  }

  return sum;
}

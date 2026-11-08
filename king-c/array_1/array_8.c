#include <stdio.h>

#define N 5

int arr_sum(int[]);

int main(void) {
  int a[N] = {1, 1, 1, 1, 1};

  printf("arr sum is %d\n", arr_sum(a));

  return 0;
}

int arr_sum(int a[]) {
  int sum = 0;
  int n = sizeof(a) / sizeof(a[0]); /*WRONG. NOT AN ARRAY LEN */
  printf("arr len is %d\n", n);     /* WRONG */
  for (int i = 0; i < n; i++) {
    sum += a[i];
  }
  return sum;
}

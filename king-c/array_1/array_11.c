#include <stdio.h>

#define N 5

int arr_sum(int[], int);

void store_zero(int[], int);

int main(void) {
  int b[N] = {1, 1, 1, 1, 1};

  store_zero(b, 2);

  printf("arr sum is %d\n", arr_sum(b, N));

  return 0;
}

void store_zero(int a[], int n) {
  for (int i = 0; i < n; i++) {
    a[i] = 0;
  }
}

int arr_sum(int a[], int n) {
  int sum = 0;
  for (int i = 0; i < n; i++) {
    sum += a[i];
  }

  return sum;
}

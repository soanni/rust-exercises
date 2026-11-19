#include <stdio.h>

int *find_largest(int[], int);

int main(void) {
  int a[5] = {111, 112, 999, 99, 88};
  printf("max is %d \n", *find_largest(a, 5));
  return 0;
}

int *find_largest(int a[], int n) {
  int max = 0, el;
  for (int i = 0; i < n; i++) {
    if (a[i] > max) {
      max = a[i];
      el = i;
    }
  }

  return &a[el];
}

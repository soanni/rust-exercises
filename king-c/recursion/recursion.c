#include <stdio.h>
#include <stdlib.h>

int fact(int n) {
  if (n <= 1) {
    return 1;
  }
  return n * fact(n - 1);
}

int power(int x, int n) { return n == 0 ? 1 : x * power(x, n - 1); }

int main(void) {
  printf("fact(3)=%d\n", fact(3));
  printf("power(2,4)=%d\n", power(2, 4));
  exit(EXIT_SUCCESS);
}

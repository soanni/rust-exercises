#include <stdio.h>
#include <stdlib.h>

void swap(int, int);

void swap(int a, int b) {
  int t = a;
  a = b;
  b = t;
  printf("swap: a = %d, b = %d\n", a, b);
}

int main() {
  int a = 1;
  int b = 2;
  swap(a, b);
  printf("main: a = %d, b = %d\n", a, b);
  return EXIT_SUCCESS;
}

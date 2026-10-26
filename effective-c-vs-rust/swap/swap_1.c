#include <stdio.h>
#include <stdlib.h>

void swap(int *pa, int *pb) {
  int t = *pa;
  *pa = *pb;
  *pb = t;
  printf("swap: a = %d, b = %d\n", *pa, *pb);
}

int main() {
  int a = 1;
  int b = 2;
  swap(&a, &b);
  printf("main: a = %d, b = %d\n", a, b);
  return EXIT_SUCCESS;
}

#include <stdio.h>

void swap(int *, int *);

int main() {
  int a = 1, b = 2;
  swap(&a, &b);
  printf("a = %d, b = %d\n", a, b);
}

void swap(int *p, int *q) {
  int t = *p;
  *p = *q;
  *q = t;
}

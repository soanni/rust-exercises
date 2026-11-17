#include <stdio.h>

void decompose(double, int *, double *);

int main() {
  int int_part;
  double d_part;
  double n = 3.14f;
  decompose(n, &int_part, &d_part);

  printf("%f is decomposed into %d and %f\n", n, int_part, d_part);
}

void decompose(double d, int *i, double *q) {
  *i = (long)d;
  *q = d - *i;
}

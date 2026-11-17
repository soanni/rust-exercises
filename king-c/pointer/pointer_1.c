#include <stdio.h>

int main() {
  int *p;
  *p = 2;
  printf("%d\n", *p); /* WRONG */
}

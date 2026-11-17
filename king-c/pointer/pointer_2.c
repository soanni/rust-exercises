#include <stdio.h>

int main() {
  int i, *p = &i;
  *p = 2;
  printf("%d\n", *p);
}

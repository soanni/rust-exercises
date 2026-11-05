#include <stdio.h>

#define SIZE (int)(sizeof(a) / sizeof(a[0]))

int main(void) {
  int a[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

  for (int i = 0; i < SIZE; i++) {
    a[i] = 0;
  }

  return 0;
}

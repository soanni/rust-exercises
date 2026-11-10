#include <stdio.h>

void f(int n) {
  static int
      nn; /* nn has a static storage duration, n has a auto storage duration */
  nn++;
}

int main(void) {}

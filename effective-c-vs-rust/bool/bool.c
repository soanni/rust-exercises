#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

bool fn1(int a) {
  if (a > 2) {
    return true;
  }

  return false;
}

void fn2(void) {
  bool a = fn1(3);
  if (a)
    puts("armed");
  else
    puts("disarmed");
}

int main() {
  fn2();

  return EXIT_SUCCESS;
}

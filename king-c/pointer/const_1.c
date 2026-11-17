#include <stdio.h>

void f(const int *);
void g(int *const);
void h(const int *const);

int main(void) {
  int i = 0;
  f(&i);
  g(&i);
  printf("after g() is called i = %d\n", i); /* i = 11 */
}

// void f(const int *p) { *p = 0; /* WRONG */ }
//
void f(const int *p) {
  int i = 1;
  p = &i; /* LEGAL as we CAN'T change the underlying CONST object that is
             referenced by p. we can change p itself but as its passed by value
             it will have no effect */
}

void g(int *const p) {
  int i = 999;
  *p = 11;
  // p = &i; /* WRONG here p itself is CONST but that's has no effect is just a
  // copy of another pointer - an argument passed when g is called */
}

// void h(const int * const p) {
//     int i = 998;
//     *p = 11; /* WRONG. Now both are protected */
//     p = &i; /* WRONG */
// }

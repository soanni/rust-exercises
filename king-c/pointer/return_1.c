#include <stdio.h>

int *middle(int[], int);
int *f(void);
// int *g(void);

int main(void) {
  int a[3] = {1, 2, 3};
  printf("middle() = %d\n", *middle(a, 3));
  printf("f() = %d\n", *f());
  //  printf("g() = %d\n", *g());
}

int *middle(int a[], int n) { return &a[n / 2]; }

int *f(void) {
  static int i = 99;
  return &i;
}

// int *g(void) {
//   int j = 111;
//   return &j; /* WRONG. local auto variable*/
// }

/* prints the sizes of types */
#include <stdio.h>

int main(void) {
  printf("sizeof(char) = %zu\n", sizeof(char));
  printf("sizeof(int) = %zu\n", sizeof(int));
  printf("sizeof(short) = %zu\n", sizeof(short));
  printf("sizeof(long) = %zu\n", sizeof(long));
  printf("sizeof(long long) = %zu\n", sizeof(long long));
  printf("sizeof(float) = %zu\n", sizeof(float));
  printf("sizeof(double) = %zu\n", sizeof(double));
  printf("sizeof(long double) = %zu\n", sizeof(long double));
  printf("sizeof(_Bool) = %zu\n", sizeof(_Bool));
  printf("sizeof(int32_t) = %zu\n", sizeof(int32_t));
  printf("sizeof(_Complex double) = %zu\n", sizeof(_Complex double));
  return 0;
}

/* examples of designated array initializers [C99] */
#include <stdio.h>

int main(void) {
  int a[10] = {[2] = 29, [3] = 30, [6] = 31};
  int b[] = {[1] = 2, [23] = 5, [11] = -1};            /* array length is 24 */
  int c[] = {1, 2, 3, 4, [4] = 5, 6, 7, 8, [15] = 16}; /* array length is 16 */
}

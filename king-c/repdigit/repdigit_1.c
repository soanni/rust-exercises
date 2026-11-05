/* program checks whether a number has a repeated digit */
/* C89 */
#include <stdio.h>

#define false 0
#define true 1
typedef int bool;

int main(void) {
  bool digits_seen[10] = {false};
  int d;
  long n;
  printf("Enter a number: ");
  scanf("%ld", &n);

  while (n > 0) {
    d = n % 10;

    if (digits_seen[d]) {
      break;
    }

    digits_seen[d] = true;

    n /= 10;
  }

  if (n > 0) {
    printf("There are repeated digits.\n");
  } else {
    printf("There are NO repeated digits.\n");
  }

  return 0;
}

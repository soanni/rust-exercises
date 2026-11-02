/* Calculate the number of digits in a number */

#include <stdio.h>

int main(void) {
  int n, count = 0;

  printf("Enter the number: ");
  scanf("%d", &n);

  do {
    n = n / 10;
    count++;
  } while (n != 0);
  printf("There are %d digits in a number enetered.\n", count);
  return 0;
}

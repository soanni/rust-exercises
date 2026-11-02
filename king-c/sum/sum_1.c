/* Sums a series of numbers */

#include <stdio.h>

int main(void) {
  long n, sum = 0;

  printf("The sum of the series of numbers\n");
  printf("Enter the numbers to sum (0 to terminate): ");

  scanf("%ld", &n);

  while (n != 0) {
    sum += n;
    scanf("%ld", &n);
  }

  printf("The sum is: %ld\n", sum);

  return 0;
}

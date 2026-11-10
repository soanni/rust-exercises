/* calculate the sum of all digits in a number */

#include <stdio.h>

int digits_sum(int);

int main(void) {
  int num;

  printf("Enter the number: ");
  scanf("%d", &num);

  printf("Sum of digits is: %d\n", digits_sum(num));

  return 0;
}

int digits_sum(int num) {
  int sum = 0;

  while (num != 0) {
    sum += num % 10;
    num /= 10;
  }

  return sum;
}

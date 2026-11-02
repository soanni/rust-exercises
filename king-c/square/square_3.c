/* Print tha table of squares */

#include <stdio.h>

int main(void) {
  int n;
  printf("The table of squares\n");
  printf("Enter the number of lines in the table: ");
  scanf("%d", &n);

  for (int square = 1, i = 1, odd = 3; i <= n; odd += 2) {
    printf("%10d%10d\n", i++, square);
    square += odd;
  }

  return 0;
}

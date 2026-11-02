/* Print tha table of squares */

#include <stdio.h>

int main(void) {
  int i, n;
  printf("The table of squares\n");
  printf("Enter the number of lines in the table: ");
  scanf("%d", &n);

  i = 1;
  while (i <= n) {
    printf("%10d%10d\n", i, i * i);
    i++;
  }

  return 0;
}

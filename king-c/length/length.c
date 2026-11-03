/* determine the length of a message including spaces and puctuation but not new
 * line character */

#include <stdio.h>

int main(void) {
  int count = 0;
  printf("Enter the string: ");
  while (getchar() != '\n')
    count++;

  printf("The length is: %d\n", count);
  return 0;
}

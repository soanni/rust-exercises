/* determine the length of a message including spaces and puctuation but not new
 * line character */

#include <stdio.h>

int main(void) {
  int count = 0;
  char ch;
  printf("Enter the string: ");
  scanf("%c", &ch);
  while (ch != '\n') {
    count++;
    scanf("%c", &ch);
  }

  printf("The length is: %d\n", count);
  return 0;
}

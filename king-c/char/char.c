/* operations on characters */

#include <ctype.h>
#include <stdio.h>

int main(void) {
  char ch;
  int i;

  i = 'a';
  printf("%d\n", i);

  ch = 'A';
  ch += 1;
  ch++;

  printf("%c\n", ch);

  ch = 'w';

  if ('a' <= ch && ch <= 'z') {
    ch = ch - 'a' + 'A';
  }

  printf("%c\n", ch);

  for (ch = 'A'; ch <= 'Z'; ch++)
    printf("%c", ch);

  printf("\n");

  ch = 'i';

  printf("%c\n", toupper(ch));
  return 0;
}

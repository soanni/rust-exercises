/* reverse using VLA - variable length arrays. C99 only */

#include <stdio.h>

int main(void) {
  int n;

  printf("How many numbers do you want to enter: ");
  scanf("%d", &n);

  int a[n], i;

  printf("Enter a sequence of numbers: ");

  for (i = 0; i < n; i++) {
    scanf("%d", &a[i]);
  }

  printf("Reverse is: ");

  /*while (i >= 0) {
        printf("%d ", a[--i]);
  }
  */

  for (int j = 0; j < n; j++) {
    printf("%d ", a[n - 1 - j]);
  }

  printf("\n");

  return 0;
}

/* find out whether number is a prime or not */
/* see how null statement is used */
#include <stdio.h>

int main(void) {
  int n, d;
  printf("Enter the number: ");
  scanf("%d", &n);

  for (d = 2; d < n && n % d != 0; d++)
    ; /* null statement */
  if (d < n)
    printf("%d is not a prime\n", n);
  else
    printf("%d is a prime\n", n);
  return 0;
}

/* test whether the entered number is a prime number */
#include <stdbool.h>
#include <stdio.h>

bool is_prime(int n) {
  if (n <= 1) {
    return false;
  }

  for (int i = 2; i * i <= n; i++) {
    if (n % i == 0) {
      return false;
    }
  }

  return true;
}

int main(void) {
  int n;

  printf("Enter the number: ");
  scanf("%d", &n);

  if (is_prime(n)) {
    printf("Is a prime\n");
  } else {

    printf("Not a prime\n");
  }

  return 0;
}

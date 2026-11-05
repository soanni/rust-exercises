/* compute the interest rates */

#include <stdio.h>

int main(void) {
  int interest, years;

  float money[5] = {100.0f, 100.0f, 100.0f, 100.0f, 100.0f};

  printf("Enter the interest rate: ");
  scanf("%d", &interest);

  printf("Enter the number of years: ");
  scanf("%d", &years);

  printf("Years\t");
  for (int i = interest; i <= interest + 4; i++) {
    printf("%d\t", i);
  }
  printf("\n");

  for (int j = 0; j < years; j++) {
    printf("%d\t", j + 1);
    for (int i = interest; i <= interest + 4; i++) {
      money[i - interest] *= (1.0f + ((float)i / 100.0f));
      printf("%.2f\t", money[i - interest]);
    }
    printf("\n");
  }

  return 0;
}

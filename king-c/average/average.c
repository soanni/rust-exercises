#include <stdio.h>

#define N 3

double average(double a, double b) { return (a + b) / 2; }

int main(void) {
  double nums[N];

  printf("Enter three numbers: ");
  for (int i = 0; i < N; i++) {
    scanf("%lf", &nums[i]);
  }

  for (int i = 0; i < N; i++) {
    for (int j = i + 1; j < N; j++) {
      printf("Average of %.2f and %.2f is %.2f\n", nums[i], nums[j],
             average(nums[i], nums[j]));
    }
  }

  return 0;
}

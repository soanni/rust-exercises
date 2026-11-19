#include <stdio.h>

#define LEN 7

void avg_sum(double[], int, double *, double *);

int main() {
  double nums[LEN] = {1.1f, 1.2f, 1.3f, 1.4f, 1.5f, 1.6f, 1.7f};
  double avg, sum;
  avg_sum(nums, LEN, &sum, &avg);
  printf("sum = %f, avg = %f\n", sum, avg);
  return 0;
}

void avg_sum(double a[], int c, double *sum, double *avg) {
  *sum = 0.0f;
  for (int i = 0; i < c; i++) {
    *sum += a[i];
  }
  *avg = *sum / c;
}

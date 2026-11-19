#include <stdio.h>

void find_two_largest(int a[], int n, int *largest, int *second_largest);

int main(void) {
  int nums[5] = {99, 88, 111, 66, 55};
  int max, max2;
  find_two_largest(nums, 5, &max, &max2);
  printf("max = %d, max2 = %d\n", max, max2);
  return 0;
}

void find_two_largest(int a[], int n, int *largest, int *second_largest) {
  int max = 0, max2 = 0;
  for (int i = 0; i < n; i++) {
    if (a[i] > max) {
      max2 = max;
      max = a[i];
    }
  }
  *largest = max;
  *second_largest = max2;
}

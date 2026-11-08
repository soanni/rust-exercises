#include <stdio.h>

int arr_sum(int[], int);

int main(void) {
  printf("arr sum is %d\n",
         arr_sum((int[]){1, 2, 3, 4, 5}, 5)); /* compound literal */
  printf("arr sum is %d\n",
         arr_sum((int[10]){8, 6}, 10)); /* compound literal */
  //         arr_sum((const int[]){8,6}, 2)); /* compound literal */

  return 0;
}

int arr_sum(int a[], int n) {
  int sum = 0;
  for (int i = 0; i < n; i++) {
    sum += a[i];
  }

  return sum;
}

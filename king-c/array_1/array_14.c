/* VLA array as an func argument */

#include <stdio.h>

int arr_sum(int n, int m, int[*][*]);

int main(void) {
  int n, m;

  printf("Enter 2 dim array dimensions (rows cols): ");
  scanf("%d", &n);
  scanf("%d", &m);

  int a[n][m];

  printf("Enter array elements: ");

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      scanf("%d", &a[i][j]);
    }
  }

  printf("arr sum is: %d\n", arr_sum(n, m, a));

  return 0;
}

/* if array in func parameter is a VLA array then dims come first in the
 * parameters list */
int arr_sum(int n, int m, int a[n][m]) {
  int sum = 0;

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      sum += a[i][j];
    }
  }

  return sum;
}

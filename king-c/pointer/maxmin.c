#include <stdio.h>

#define N 10

void maxmin(int[], int, int *, int *);

int main(void) {
  int b[N];
  int big, small;

  printf("Enter 10 numbers: \n");
  for (int i = 0; i < N; i++) {
    scanf("%d", &b[i]);
  }

  maxmin(b, N, &small, &big);

  printf("Largest: %d\n", big);
  printf("Samllest: %d\n", small);

  return 0;
}

void maxmin(int arr[], int n, int *min, int *max) {
  *min = arr[0];
  *max = arr[0];

  for (int i = 1; i < n; i++) {
    if (arr[i] < *min)
      *min = arr[i];
    if (arr[i] > *max)
      *max = arr[i];
  }
}

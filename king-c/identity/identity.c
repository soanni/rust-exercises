/* identity matrix 10x10 */

#include <stdio.h>

#define N 10

int main(void) {
  int rows, cols;
  double matrix[N][N] = {0};

  for (rows = 0; rows < N; rows++) {
    for (cols = 0; cols < N; cols++) {
      if (rows == cols) {
        matrix[rows][cols] = 1;
      }
    }
  }

  // check
  //
  for (rows = 0; rows < N; rows++) {
    for (cols = 0; cols < N; cols++)
      printf("%0.2f\t", matrix[rows][cols]);
    printf("\n");
  }

  return 0;
}

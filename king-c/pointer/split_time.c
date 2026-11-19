#include <stdio.h>

void split_time(long total_sec, int *hr, int *min, int *sec);

int main(void) {
  int h, m, s;

  split_time(3661, &h, &m, &s);

  printf("hr = %d, min = %d, sec = %d\n", h, m, s);

  return 0;
}

void split_time(long total_sec, int *hr, int *min, int *sec) {
  *hr = total_sec / 3600;
  *min = (total_sec % 3600) / 60;
  *sec = total_sec - (*hr * 3600) - (*min * 60);
}

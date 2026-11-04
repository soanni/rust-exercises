#include <stdio.h>
#include <string.h>

typedef unsigned char *byte_pointer;

void show_bytes(byte_pointer start, size_t n) {
  printf("*** byte_pointer is %p ***", (void *)start);
  for (int i = 0; i < n; i++) {
    printf("%.2x", start[i]);
  }
  printf("\n");
}

void show_int(int x) { show_bytes((byte_pointer)&x, sizeof(int)); }

void show_float(float x) { show_bytes((byte_pointer)&x, sizeof(float)); }

void show_pointer(void *x) { show_bytes((byte_pointer)&x, sizeof(void *)); }

int main(void) {
  int n = 12345;
  float f = (float)n;
  int *p = &n;

  const char *m = "mnopqr";

  show_int(n);
  show_float(f);
  show_pointer(p);
  show_bytes((byte_pointer)m, strlen(m));
}

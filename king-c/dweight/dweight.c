/* calculating the dimensional weight of a package L x W x H */

#include <stdio.h>

int main(void) {
  int length, width, height, volume, weight;
  length = 12;
  width = 8;
  height = 10;
  // cubic inches
  volume = length * width * height;
  // integer division is rounding to lower whole value so adding 165 to round up
  weight = (volume + 165) / 166;
  printf("Dims: %dx%dx%d\n", length, width, height);
  printf("Vol: %d\n", volume);
  printf("Weight: %d\n", weight);
  return 0;
}

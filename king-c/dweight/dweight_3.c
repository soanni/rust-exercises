/* calculating the dimensional weight of a package L x W x H
 * based on user input values for dimensions
 */

#include <stdio.h>

int main(void) {
  int length, width, height, volume, weight;
  printf("Enter the length of a box: ");
  scanf("%d", &length);
  printf("Enter the width of a box: ");
  scanf("%d", &width);
  printf("Enter the height of a box: ");
  scanf("%d", &height);
  //  cubic inches
  volume = length * width * height;
  // integer division is rounding to lower whole value so adding 165 to round up
  weight = (volume + 165) / 166;
  printf("Dims: %dx%dx%d\n", length, width, height);
  printf("Vol: %d\n", volume);
  printf("Weight: %d\n", weight);
  return 0;
}

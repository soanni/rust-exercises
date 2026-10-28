/* Fahrenheit to Celsius conversion */

#include <stdio.h>

#define FREEZING_PT 32.0f
#define SCALE_FACTOR (5.0f / 9.0f)

int main(void) {
  float fah, cels;
  printf("Enter the Fahrenheit temp: ");
  scanf("%f", &fah);
  cels = (fah - FREEZING_PT) * SCALE_FACTOR;
  printf("Celsius is: %.1f\n", cels);
  return 0;
}

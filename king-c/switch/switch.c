#include <stdio.h>

int main(void) {
  int grade;

  printf("Enter the grade: ");
  scanf("%d", &grade);

  switch (grade) {
  case 9:
  case 8:
  case 7:
  case 6:
  case 5:
    printf("Excellent");
  case 4:
    printf("Good");
  case 3:
    printf("Normal");
  case 2:
    printf("Bad");
    break;
  case 1:
    printf("Aweful");
  default:
    printf("Unknown");
  }

  return 0;
}

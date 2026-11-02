/* checkbook with goto */
#include <stdio.h>

int main(void) {
  float sum = 0.0f, value;
  int command;
  printf("Commands: 0=clear, 1=credit, 2=debit, 3=balance, 4=exit\n");
  for (;;) {
    printf("Enter command: ");
    scanf("%d", &command);
    switch (command) {
    case 0:
      sum = 0.0f;
      break;
    case 1:
      printf("Enter ammount of credit: ");
      scanf("%f", &value);
      sum += value;
      break;
    case 2:
      printf("Enter ammount of debit: ");
      scanf("%f", &value);
      sum -= value;
      break;
    case 3:
      printf("Current balance: $%.2f\n", sum);
      break;
    case 4:
      goto label;
    default:
      printf("Commands: 0=clear, 1=credit, 2=debit, 3=balance, 4=exit\n");
      break;
    }
  }
label:; /* rare case of using a null statement */
}

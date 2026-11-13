#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define MAX_NUMBER 20

void initialize_random_generator(void);
int generate_secret_number(void);
void read_guesses(int secret_number);

int main(void) {
  char command;
  int secret_number;

  printf("Guess the secret number between 1 and %d\n\n", MAX_NUMBER);

  initialize_random_generator();

  do {
    secret_number = generate_secret_number();
    printf("A new number has been chosen\n");
    read_guesses(secret_number);
    printf("Play again ? Y|N ");
    scanf(" %c", &command);
  } while (command == 'y' || command == 'Y');
}

void initialize_random_generator(void) { srand((unsigned)time(NULL)); }

int generate_secret_number(void) {
  int secret_number = rand();
  secret_number = secret_number % MAX_NUMBER + 1;
  return secret_number;
}

void read_guesses(int secret_number) {
  int guess, num_guesses = 0;

  for (;;) {
    num_guesses++;
    printf("Enter guess: ");
    scanf("%d", &guess);

    if (guess == secret_number) {
      printf("You won in %d guesses!\n", num_guesses);
      return;
    } else if (guess < secret_number) {
      printf("Too low, try again\n");
    } else {
      printf("Too high, try again\n");
    }
  }
}

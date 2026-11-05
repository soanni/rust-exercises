/* Ace of spades */
/* clubs, diamonds, hearts, spades */
/* 2,3,4,5,6,7,8,9,10,jack,queen,king,ace */

#include <stdbool.h> /* C99 */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define NUM_RANKS 13
#define NUM_SUITS 4

int main(void) {
  int cards, suit, rank;

  const char ranks[NUM_RANKS] = {'2', '3', '4', '5', '6', '7', '8',
                                 '9', 't', 'j', 'q', 'k', 'a'};
  const char suits[NUM_SUITS] = {'d', 'h', 's', 'c'};
  bool in_hands[NUM_SUITS][NUM_RANKS] = {false};

  srand((unsigned)time(NULL));

  printf("How many cards would like: ");
  scanf("%d", &cards);

  while (cards > 0) {
    suit = rand() % NUM_SUITS;
    rank = rand() % NUM_RANKS;
    if (!in_hands[suit][rank]) {
      in_hands[suit][rank] = true;
      printf(" %c%c", ranks[rank], suits[suit]);
      cards--;
    }
  }

  printf("\n");

  return 0;
}

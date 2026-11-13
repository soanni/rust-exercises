#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

void read_cards(void);
void analyze_hand(void);
void print_results(void);

#define NUM_SUITS 4
#define NUM_RANKS 13
#define NUM_CARDS 5

int num_in_suit[NUM_SUITS];
int num_in_rank[NUM_RANKS];
bool straight, flush, four, three;
int num_pairs;

int main(void) {
  for (;;) {
    read_cards();
    analyze_hand();
    print_results();
  }
}

void print_results(void) {
  if (straight && flush) {
    printf("Straight flush.\n");
  } else if (flush) {
    printf("Flush.\n");
  } else if (straight) {
    printf("Straight.\n");
  } else if (three && num_pairs == 1) {
    printf("Full house.\n");
  } else if (three) {
    printf("Three of a kind.\n");
  } else if (four) {
    printf("Four of a kind.\n");
  } else if (num_pairs == 2) {
    printf("Two pairs.\n");
  } else if (num_pairs == 1) {
    printf("Pair.\n");
  } else {
    printf("High card.\n");
  }
}

void analyze_hand(void) {
  int rank, suit;
  int num_conseq = 0;

  num_pairs = 0;

  straight = false;
  flush = false;
  four = false;
  three = false;

  /* check for flash */
  for (suit = 0; suit < NUM_SUITS; suit++) {
    if (num_in_suit[suit] == NUM_CARDS) {
      flush = true;
    }
  }

  /* check for straight */

  rank = 0;
  while (num_in_rank[rank] == 0) {
    rank++;
  }

  for (; rank < NUM_RANKS && num_in_rank[rank] > 0; rank++) {
    num_conseq++;
  }

  if (num_conseq == NUM_CARDS) {
    straight = true;
  }

  /* check for four, three of a kind, pairs */
  for (rank = 0; rank < NUM_RANKS; rank++) {
    if (num_in_rank[rank] == 4)
      four = true;
    if (num_in_rank[rank] == 3)
      three = true;
    if (num_in_rank[rank] == 2) {
      num_pairs++;
    }
  }
}

void read_cards(void) {
  int cards = 0;
  char ch, ch_rank, ch_suit;
  bool bad_card;
  bool card_exists[NUM_RANKS][NUM_SUITS];
  int rank, suit;

  for (int i = 0; i < NUM_RANKS; i++) {
    num_in_rank[i] = 0;
    for (int j = 0; j < NUM_SUITS; j++) {
      card_exists[i][j] = false;
    }
  }

  for (int i = 0; i < NUM_SUITS; i++) {
    num_in_suit[i] = 0;
  }

  while (cards < NUM_CARDS) {
    bad_card = false;
    printf("Enter a card: ");
    ch_rank = getchar();
    switch (ch_rank) {
    case '0':
      exit(EXIT_SUCCESS);
    case '2':
      rank = 0;
      break;
    case '3':
      rank = 1;
      break;
    case '4':
      rank = 2;
      break;
    case '5':
      rank = 3;
      break;
    case '6':
      rank = 4;
      break;
    case '7':
      rank = 5;
      break;
    case '8':
      rank = 6;
      break;
    case '9':
      rank = 7;
      break;
    case 't':
    case 'T': /* 10 */
      rank = 8;
      break;
    case 'j':
    case 'J': /* jack */
      rank = 9;
      break;
    case 'q':
    case 'Q': /* queen */
      rank = 10;
      break;
    case 'k':
    case 'K': /* king */
      rank = 11;
      break;
    case 'a':
    case 'A': /* ace */
      rank = 12;
      break;
    default:
      bad_card = true;
    }

    ch_suit = getchar();

    switch (ch_suit) {
    case 'c':
    case 'C': /* clubs */
      suit = 0;
      break;
    case 'd':
    case 'D': /* diamonds */
      suit = 1;
      break;
    case 'h':
    case 'H':
      suit = 2;
      break;
    case 's':
    case 'S':
      suit = 3;
      break;
    default:
      bad_card = true;
    }

    while ((ch = getchar()) != '\n') {
      if (ch != ' ') {
        bad_card = true;
      }
    }

    if (bad_card) {
      printf("Bad card. Ignored.\n");
    } else if (card_exists[rank][suit]) {
      printf("Duplicate card. Ignored.!\n");
    } else {
      card_exists[rank][suit] = true;
      num_in_rank[rank]++;
      num_in_suit[suit]++;
      cards++;
    }
  }
}

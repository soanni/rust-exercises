#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define STACK_SIZE 100

/* external variables */
int top = 0;
int stack[STACK_SIZE];

int main(void) {}

void make_empty(void) { top = 0; }

bool is_empty(void) { return top == 0; }

bool is_full(void) { return top == STACK_SIZE; }

void stack_overflow(void) {
  printf("!!! stack overflow !!!");
  exit(1);
}

void stack_underflow(void) {
  printf("!!! stack underflow !!!");
  exit(1);
}

void push(int n) {
  if (is_full()) {
    stack_overflow();
  } else {
    stack[top++] = n;
  }
}

int pop(void) {
  if (is_empty()) {
    stack_underflow();
  }
  return stack[--top];
}

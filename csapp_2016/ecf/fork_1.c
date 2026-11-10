#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <unistd.h>

void unix_error(char *);

pid_t Fork(void);

int main(void) {
  Fork();
  Fork();
  printf("helllo\n");
  exit(0);
}

void unix_error(char *msg) { /* unix style error */
  fprintf(stderr, "%s: %s\n", msg, strerror(errno));
  exit(0);
}

pid_t Fork(void) {
  pid_t pid;

  if ((pid = fork()) < 0) {
    unix_error("fork error.");
  }

  return pid;
}

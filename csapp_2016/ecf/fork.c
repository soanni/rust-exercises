#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <unistd.h>

void unix_error(char *);

pid_t Fork(void);

int main(void) {
  int x = 1;
  pid_t pid;

  pid = Fork();

  if (pid == 0) { /* child process */
    printf("child: x = %d\n", --x);
    exit(0);
  }

  printf("parent: x = %d\n", ++x);
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

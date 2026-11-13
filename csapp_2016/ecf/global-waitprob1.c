#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/wait.h>
#include <unistd.h>

void unix_error(char *);

pid_t Fork(void);

int main(void) {
  int status;
  pid_t pid;

  printf("Start\n");
  pid = Fork();
  printf("%d\n", !pid);

  if (pid == 0) {
    printf("Child\n");
  } else if ((waitpid(-1, &status, 0) > 0) && (WIFEXITED(status) != 0)) {
    printf("%d\n", WEXITSTATUS(status));
  }
  printf("Stop\n");
  exit(2);
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

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/wait.h>
#include <unistd.h>

#define N 5

void unix_error(char *);

pid_t Fork(void);

int main(void) {
  int status, i;

  pid_t pid[N], retpid;

  /* Parent creates child processes */
  for (i = 0; i < N; i++) {
    if ((pid[i] = Fork()) == 0) {
      exit(100 + i);
    }
  }

  i = 0;
  /* Parent reaps child processes in the same order they were created */
  while ((retpid = waitpid(pid[i++], &status, 0)) > 0) {
    if (WIFEXITED(status)) {
      printf("child %d terminated normally with exited status %d\n", retpid,
             WEXITSTATUS(status));
    } else {
      printf("child %d terminated abnormally\n", retpid);
    }
  }

  if (errno != ECHILD) {
    unix_error("waitpid error");
  }

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

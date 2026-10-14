#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

./server_bin &
pid1=$!

wait $pid1
exit_code1=$?

echo "Task 1 (PID $pid1) exited with code $exit_code1"

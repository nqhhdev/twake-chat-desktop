#!/usr/bin/env bash
# Runs a command with its output in dev/.<name>.log and its exit code in dev/.<name>.exit,
# so a long build can be watched from another process.
name=$1; shift
cd "$(dirname "$0")/.."
rm -f "dev/.$name.exit"
"$@" > "dev/.$name.log" 2>&1
code=$?
echo "$code" > "dev/.$name.exit"
tail -5 "dev/.$name.log"
exit $code

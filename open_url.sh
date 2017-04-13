#!/usr/bin/env bash

# Opens a URL in the local SSH client machine's browser

cat <(echo "$1") > /dev/tcp/localhost/37705

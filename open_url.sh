#!/usr/bin/env bash

# Opens a URL in the local SSH client machine's browser
#
# Irssi openurl.pl configuration:
#   /set openurl_app_http /home/user/path/to/open_url.sh "$1"

cat <(echo "$1") > /dev/tcp/localhost/37705

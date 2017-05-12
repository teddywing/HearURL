HearURL
=======

A URL launcher. The program listens over TCP and opens URLs it receives in a
local browser.

This makes it easy to open URLs from SSH sessions. When using programs like
Irssi and Mutt over SSH, a URL can be sent to HearURL to be opened
locally.


## Usage
Start HearURL in the background:

	$ nohup hearurl --browser Firefox > /tmp/hearurl.out 2>&1&

(Or use an init system like `systemd` or `launchd`.)

To use over SSH, set up remote forwarding to HearURL's local port (by default
set to `37705`):

	$ ssh -R 34254:localhost:37705 user@example.com

On the remote server, send a URL to `localhost:34254`, and it will open
locally.

A shell script ([`open_url.sh`][1]) is provided to simplify sending URLs over
the socket. Here's an example:

	$ ./open_url.sh https://duckduckgo.com/


## Install
**Note:** Only works for Mac OS X since the program delegates to
[`open`][2].

A binary built for Mac OS X is available on the [releases][3] page. Download the
binary and put it in your `PATH`.

To compile from source:

	$ cargo install --git https://github.com/teddywing/HearURL.git --root /usr/local


## Uninstall

	$ cargo uninstall --root /usr/local hearurl


## License
Copyright © 2017 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).


[1]: ./open_url.sh
[2]: https://developer.apple.com/legacy/library/documentation/Darwin/Reference/ManPages/man1/open.1.html
[3]: https://github.com/teddywing/HearURL/releases

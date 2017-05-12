#!/bin/sh

VERSION=0.1.0

cargo build --release

cd target/release/
tar cjvf "hearurl_${VERSION}_osx_amd64.tar.bz2" hearurl

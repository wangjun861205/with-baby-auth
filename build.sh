#!/bin/sh

cargo build --release
docker build --force-rm -t with-baby-auth:v0.1 .
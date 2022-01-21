#!/usr/bin/env bash
docker build -t myrust .
docker run -it --rm -v "$PWD:/mnt" myrust 

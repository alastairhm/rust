#!/usr/bin/env bash
#docker build -t alastairhm/myrust:latest .
docker run -it --rm -v "$PWD:/mnt" alastairhm/myrust 

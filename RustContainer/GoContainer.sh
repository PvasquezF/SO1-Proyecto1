#!/bin/bash
docker build -t rustcontainer .
docker run -d -p 8888:8888 rustcontainer
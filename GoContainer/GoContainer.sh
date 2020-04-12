#!/bin/bash
docker build -t gocontainer .
docker run -d -p 8081:8081 gocontainer
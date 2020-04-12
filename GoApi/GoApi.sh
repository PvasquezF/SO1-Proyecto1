#!/bin/bash
docker build -t goapi .
docker run -d -p 8080:8080 goapi
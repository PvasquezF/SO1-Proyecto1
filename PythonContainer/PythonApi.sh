#!/bin/bash
docker build -t pythoncontainer .
docker run -d -p 5000:5000 pythoncontainer
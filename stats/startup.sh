#!/bin/sh

touch .env
./chm migration run
./stats --port 7000

#!/bin/sh

./sqlx migrate run
./posts --port 5000

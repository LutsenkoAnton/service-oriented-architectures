#!/bin/sh

./sqlx migrate run
./users --port 3000

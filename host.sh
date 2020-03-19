#!/bin/bash
while :
do
	echo "Starting up the server"
	pkill realm-one
	cp ../config.ron resources/
	cp ../Cargo.toml .
	./target/release/realm-one server
	sleep 1
done

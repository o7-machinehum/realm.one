#!/bin/bash
while :
do
	echo "Starting up the server"
	./target/release/realm-one server
	sleep 1
done

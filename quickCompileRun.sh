#!/bin/zsh

if [ -z "$1" ]; then
	echo "Choose a file to compile man!"
	exit 1
fi

export FILE=$1
g++ $FILE -Wall -Wextra -Werror -ggdb -o $FILE.exe -Wno-deprecated-declarations -O0 && $FILE.exe

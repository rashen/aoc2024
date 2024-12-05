#!/usr/bin/bash

if [[ ! ${1} =~ ^[0-9]+$ ]]; then
    echo "Input ${1} is not a day"
    exit 1
fi

if [[ -f .session ]]; then
    SESSION=$(<.session)
    mkdir -p input
    curl https://adventofcode.com/2024/day/${1}/input -q --compressed -H "Cookie: session=$SESSION" >input/day${1}.txt
fi

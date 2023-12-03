#!/bin/bash

export name=$1 
export number=$2

if [ ! $number ] || [ ! $name ]; then 
    echo -e "Usage: ./new.sh <name> <number>"
    exit 1
fi

if [ -z $($number | grep "^-\?[0-9]+$") ]; then 
    echo -e "Expected int, got $number"
    exit 1
fi

cd /Users/ta4h1r/Dev/rust_tutorials
cargo new $name 

mv $name "${number}_${name}" && cd "${number}_${name}"
 
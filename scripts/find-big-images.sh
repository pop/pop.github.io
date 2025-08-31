#!/bin/sh

find . -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' \
    | xargs -I{} -n1 identify {} \
    | grep -E '[[:digit:]]{4}x' \
    | grep -Ev '10[[:digit:]]{2}x' \
    | cut -d ' ' -f 1,3

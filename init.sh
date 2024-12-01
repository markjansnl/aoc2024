#!/bin/sh

for day in src/days/day*
do
    printf '%s' "0" > $day/input.txt
done

printf '%s' "$1" > .session
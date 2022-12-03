#!/bin/bash
#
# get your session ID from Chrome's devtools and replace it below. then use like:
#
# $ ./pull.sh > input.txt

SESSION=YourSessionIDHere

DAY=$(date +%-d)
YEAR=$(date +%Y)

URL=https://adventofcode.com/$YEAR/day/$DAY/input

curl --cookie "session=$SESSION" \
     -H "User-Agent: custom bash script with curl, by github.com/jasonincanada" \
     $URL

#!/bin/bash
./hyperfine --warmup 3 -N --export-json report.json \
'./solve 2 3 5' \
'./solve 3 3 6' \
'./solve 3 4 7' \
'./solve 4 4 9' \
'./solve 4 5 11' \
'./solve 5 5 14' \
'./solve 5 6 16' \
'./solve 6 6 19' \
'./solve 6 7 22' \
'./solve 7 7 26' \
'./solve 7 8 29' \
'./solve 8 8 33' \
'./solve 8 9 37'
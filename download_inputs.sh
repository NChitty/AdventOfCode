#!/bin/bash

# Define variables
YEAR=$1
DAY=$2
BASE_URL="https://adventofcode.com/${YEAR}/day/${DAY}"
OUTPUT_DIR="${YEAR}/src/bin/day${DAY}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Fetch the main page and extract the first <code> block
curl -s -H "Cookie: session=${AOC_SESSION}" "${BASE_URL}" | \
    awk 'BEGIN { in_code=0 }
        /<code>/ { in_code=1; sub(/.*<code>/, ""); }
        /<\/code>/ { in_code=0; sub(/<\/code>.*/, ""); print; exit }
        { if (in_code) print }' > "${OUTPUT_DIR}/sample.txt"

if [[ -s "${OUTPUT_DIR}/sample.txt" ]]; then
    echo "Sample text saved to ${OUTPUT_DIR}/sample.txt"
else
    echo "Failed to extract <code> block or <code> block not found."
fi

# Fetch the input data
curl -s -H "Cookie: session=${AOC_SESSION}" "${BASE_URL}/input" -o "${OUTPUT_DIR}/input.txt"

if [[ -s "${OUTPUT_DIR}/input.txt" ]]; then
    echo "Input data saved to ${OUTPUT_DIR}/input.txt"
else
    echo "Failed to fetch input data."
fi

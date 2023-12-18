#!/bin/bash

output_file="random_incremental_file.txt"
file_size_mb=256

# Calculate the number of lines needed to achieve the desired size
num_lines=$((file_size_mb * 1024 * 1024 / 4)) # Assuming each number takes 4 bytes

# Generate incremental numbers and save to the file
seq 1 "$num_lines" >"/tmp/$output_file"

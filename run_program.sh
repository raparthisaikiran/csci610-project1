#!/bin/bash

# Ask user for time interval
read -p "Enter time interval in seconds or 'w' for one week: " interval

# Check if input is a valid number or 'w'
if ! [[ $interval =~ ^[0-9]+$ || $interval = "w" ]]; then
  echo "Invalid input. Please enter a valid number or 'w' for one week."
  exit 1
fi

# Determine the number of seconds in the interval
if [ $interval = "w" ]; then
  interval_seconds=$((7*24*60*60))
else
  interval_seconds=$interval
fi

# Loop infinitely
while true; do
  # Run Rust program
  cargo run

  # Wait for specified interval
  sleep $interval_seconds
done

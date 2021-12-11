#!/bin/bash

DAY=$1

cp "input/input$DAY.txt" "input/tmp$DAY.txt"
cp "input/bck$DAY.txt" "input/input$DAY.txt"
mv "input/tmp$DAY.txt" "input/bck$DAY.txt"

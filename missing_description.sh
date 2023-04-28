#!/bin/bash

ALL_FILES="$(ag -G.rs -l | sort)"
WITH_FILES="$(ag "///" -G.rs -l | sort)"
comm -3 <(echo "$ALL_FILES") <(echo "$WITH_FILES")

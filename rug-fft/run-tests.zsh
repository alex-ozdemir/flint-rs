#!/usr/bin/env zsh
lim=$1
echo fft,logsize,time
for fft in ct naive br; do
    for i in $(seq 1 20); do
        t=$(timeout "${lim}s" cargo run --release --example bench-fft -q -- $i $fft)
        if [ $? -eq 0 ]; then
            echo $fft,$i,$t
        else
            break
        fi
    done
done

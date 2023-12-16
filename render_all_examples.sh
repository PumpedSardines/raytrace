#!/bin/bash

echo "Running all examples"
echo "ball"
cargo run --release --example ball
echo "balls"
cargo run --release --example balls
echo "teapot"
cargo run --release --example teapot
echo "triangle"
cargo run --release --example triangle

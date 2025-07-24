#!/usr/bin/env bash
cmake -B build && cmake --build build && gdb -ex run --args ./build/AuroraVisualizer
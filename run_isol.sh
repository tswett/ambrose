#!/bin/bash

taskset -cp 3 $$
exec cargo run "$@"

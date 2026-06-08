#!/bin/bash

workers=(worker1 worker2 worker3)

hostname=$(hostname)

for worker in "${workers[@]}"; do
    dir="target/debug/$worker"
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
        echo "Created $dir"
    else
        echo "$dir already exists"
    fi
    cp target/debug/worker "$dir/worker"
    echo "Copied worker binary to $dir"

    sed "s|{WORKER_ID}|$worker|g; s|{HOSTNAME}|$hostname|g" env.worker > "$dir/.env"
    echo "Copied env.worker to $dir/.env (WORKER_ID=$worker, HOSTNAME=$hostname)"
done

coordinator_dir="target/debug/coordinator1"
if [ ! -d "$coordinator_dir" ]; then
    mkdir -p "$coordinator_dir"
    echo "Created $coordinator_dir"
else
    echo "$coordinator_dir already exists"
fi

cp target/debug/coordinator "$coordinator_dir/coordinator"
echo "Copied coordinator binary to $coordinator_dir"

sed "s|{ROOT_FOLDER}|$(pwd)|g" env.coordinator > "$coordinator_dir/.env"
echo "Copied env.coordinator to $coordinator_dir/.env (ROOT_FOLDER=$(pwd))"

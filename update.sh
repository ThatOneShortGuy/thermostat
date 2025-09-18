#!/bin/bash

git pull

nohup setsid /bin/bash ./deploy-client.sh release --no-run \
    >"${PWD}/deploy-client.log" 2>&1 </dev/null &

echo "==> Compiling Release Build"
RUSTFLAGS='-C target-cpu=native' cargo build --release --bin server

sudo cp ./thermostat_server.service /etc/systemd/system/thermostat_server.service
sudo systemctl daemon-reload
sudo systemctl enable thermostat_server.service
sudo systemctl restart thermostat_server.service

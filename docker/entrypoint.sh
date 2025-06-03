#!/bin/sh
#
#   This script runs two processes:
#   1)  anvil-zksync on port 8011  (runs in background)
#   2)  zksync-airbender on port 3030 (runs in foreground)
#


# Start anvil-zksync in the background, listening on port 8011
exec /app/anvil-zksync --port 8011 --use-boojum --boojum-bin-path /app/app.bin &

# Then start zksync-airbender in the foreground on port 3030
exec /app/zksmith --host-port 0.0.0.0:3030 --anvil-url http://localhost:8011 --zksync-os-bin-path /app/app.bin
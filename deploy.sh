#!/usr/bin/env bash

set -euo pipefail

cd lambda || {
  echo must be run in the root directory
  exit 1
}
cargo lambda build --release --target x86_64-unknown-linux-gnu

cd ../cdk
cdk deploy

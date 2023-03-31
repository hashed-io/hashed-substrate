#!/bin/bash

INSTALL_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $INSTALL_ROOT
cd ../
cd pallets/

find . -name "Cargo.toml" -exec cargo clippy --all-targets --all-features --manifest-path {} \;

#

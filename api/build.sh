#!/bin/sh

if [ -f ../Cargo.lock ]; then
    echo Copying Cargo.lock files ...
    cp ../Cargo.lock ../api/Cargo.lock
    cp ../Cargo.lock ../bin/Cargo.lock
    cp ../Cargo.lock ../lib/Cargo.lock
fi

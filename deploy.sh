#!/bin/sh

docker build \
    --tag docker.pkg.github.com/progamesigner/rfgames-backend/server:1.0 \
    .

docker push docker.pkg.github.com/progamesigner/rfgames-backend/server:1.0

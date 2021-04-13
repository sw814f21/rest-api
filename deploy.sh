#!/bin/bash

function build {
    cargo build --target x86_64-unknown-linux-musl --release --locked --bin smiley_rest_api
}

function upload {
    rsync --info=progress2 ./target/x86_64-unknown-linux-musl/release/smiley_rest_api p8:/var/smiley_rest_api/smiley_rest_api

    ssh p8 sudo -S systemctl restart smiley_rest_api.service
}

echo "Building the binary"
build
echo "Uploading the binary"
upload
echo "Done with deployment."
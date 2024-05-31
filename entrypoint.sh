#!/bin/sh

cargo test --test private
echo "\n##### Open Orders Raport #####\n"
cat kraken_private_api_response.json
echo "\n##############################\n"

cargo test --test public
echo "\n##### Traiding Pair Raport #####\n"
cat kraken_api_response.json
echo "\n##############################\n"

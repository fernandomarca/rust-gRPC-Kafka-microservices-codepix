#!/bin/bash

apt-get update && apt-get upgrade -y
apt-get install -y build-essential

apt-get install -y curl
apt-get install -y protobuf-compiler
apt-get install -y cmake 
apt-get install -y openssl 
apt-get install -y libssl-dev 
apt-get install -y libsasl2-dev
apt-get install -y pkg-config
apt-get install -y libzstd-dev
apt-get install -y librdkafka-dev
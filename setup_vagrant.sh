#!/bin/bash
# Copy ssh of your host machine
mkdir -p  ssh
cp ~/.ssh/id_rsa ./ssh/id_rsa
cp ~/.ssh/known_hosts ./ssh/known_hosts
# Build everything
vagrant up

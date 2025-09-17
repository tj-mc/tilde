#!/bin/bash

# Get current version from Cargo.toml
grep -oP 'version = "\K[^"]+' Cargo.toml | head -1
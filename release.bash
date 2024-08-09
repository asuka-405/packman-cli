#!/bin/bash

rm -rf dist
mkdir dist
cp -r src dist
cp -r Cargo.toml dist
cp -r Cargo.lock dist
cp -r README.md dist
cp -r LICENSE dist

tar -czf packman.tar.gz dist

rm -rf dist/*
cp -r target/release/packman dist
mv packman.tar.gz dist
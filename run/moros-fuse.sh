#!/bin/sh

img="disk.img"
path="/tmp/mushix"

# pip install fusepy
mkdir -p $path
echo "Mounting $img in $path"
python run/mushix-fuse.py $img $path

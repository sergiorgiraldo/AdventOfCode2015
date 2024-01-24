#!/bin/sh

if cargo test -q; then
    :
else
    echo "Tests failed"
    exit 1
fi    

git add --all . 

git commit -S -m 'feat!: day '"$1"' completed'

git push -u origin HEAD

gh pr create --fill --base main

gh pr merge --merge --auto
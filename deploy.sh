#!/bin/sh

if cargo test -q; then
    git add --all . 

    git commit -S -m 'feat!: day '"$1"' completed'

    git push -u origin HEAD

    gh pr create --fill --base main

    gh pr merge --merge --auto
else
    echo "Test(s) failed"
    exit 1
fi    

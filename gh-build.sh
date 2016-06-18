#!/bin/bash
VENV='.venv'

git stash --all
rm -rf output/ .cache/

source $VENV/bin/activate

sass --update theme/style.scss:theme/style.css
acrylamid compile

rm -rf /tmp/build_output
mv output /tmp/build_output

git checkout master

rm -rf ./*
mv /tmp/build_output/* ./

git add .
git commit -m "Site compiled on `date`"
git push origin master
git checkout source
git stash pop

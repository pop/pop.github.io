#!/bin/bash
VENV='.venv'

source $VENV/bin/activate
acrylamid compile
sass --update theme/style.scss:theme/style.css
rm -rf /tmp/build_output
mv output /tmp/build_output
git checkout master
rm -rf ./*
mv /tmp/build_output/* ./
git add .
git commit -m "Site compiled on `date`"
git push origin master
git checkout source

#!/bin/sh

poetry export --without-hashes -f requirements.txt -o requirements.txt

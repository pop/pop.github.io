#!/bin/sh

img=$1
magick $img -resize 1024x $img

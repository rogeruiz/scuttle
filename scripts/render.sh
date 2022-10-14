#!/bin/bash

for incoming_file in "$@"
do
  if [ -f "${incoming_file}" ]
  then
    echo "creating a PNG file for ${incoming_file}."
    plantuml "${incoming_file}"
  fi
done

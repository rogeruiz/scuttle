#!/bin/bash

# If this gets invoked without any arguments, then we need to build a list of
# DSL files that need to be rendered. At least that first idea here.

# If there is a file that gets passed in as the first argument, then we can just assume that it's the only thing we need to render.

## Let's find out what the variables are here
incoming_file=${1}

latest_structurizr_cli_docker_sha='d2e6a0105943'

if ! { docker images -q | rg "${latest_structurizr_cli_docker_sha}" > /dev/null; } 2>&1
then
  docker pull structurizr/cli
fi

if [ -f "${incoming_file}" ]
then
  echo "exporting PlantUML file for ${incoming_file}."
  docker run \
    -u "$(id -u):$(id -g)" \
    -i \
    --rm \
    -v "${PWD}:/usr/local/structurizr" \
    structurizr/cli export  -workspace "${incoming_file}" -format "plantuml/structurizr" -output ./out
fi

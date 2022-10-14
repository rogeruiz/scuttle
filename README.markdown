# Scuttle 2.0

![Scuttle](scuttle.jpg)

```shell
       ___           ___           ___                                                     ___
      /\__\         /\__\         /\  \                                                   /\__\
     /:/ _/_       /:/  /         \:\  \         ___           ___                       /:/ _/_
    /:/ /\  \     /:/  /           \:\  \       /\__\         /\__\                     /:/ /\__\
   /:/ /::\  \   /:/  /  ___   ___  \:\  \     /:/  /        /:/  /      ___     ___   /:/ /:/ _/_
  /:/_/:/\:\__\ /:/__/  /\__\ /\  \  \:\__\   /:/__/        /:/__/      /\  \   /\__\ /:/_/:/ /\__\
  \:\/:/ /:/  / \:\  \ /:/  / \:\  \ /:/  /  /::\  \       /::\  \      \:\  \ /:/  / \:\/:/ /:/  /
   \::/ /:/  /   \:\  /:/  /   \:\  /:/  /  /:/\:\  \     /:/\:\  \      \:\  /:/  /   \::/_/:/  /
    \/_/:/  /     \:\/:/  /     \:\/:/  /   \/__\:\  \    \/__\:\  \      \:\/:/  /     \:\/:/  /
      /:/  /       \::/  /       \::/  /         \:\__\        \:\__\      \::/  /       \::/  /
      \/__/         \/__/         \/__/           \/__/         \/__/       \/__/         \/__/

```

This project is a re-write of the original version of Scuttle of Diagrams as
Code (**DGaC**). Since the work of Simon Brown and the C4 Model and Structurizr,
a lot has changed in the world of DGaC. This project is here to help the
development of these types of diagrams in an automated way with a reduced
feedback loop.

## Installation

This project relies on a number of other CLI tools. Below is a list. This list
is checked whenever Make commands are run. For now, please refer to those
project's installation instructions in order to install them. On macOS, these
tools can easily be installed using Homebrew.

- docker
    - structurizr-cli
        - _This is automatically pulled with the latest release on DockerHub
          rather than Homebrew_.
- watchman
- plantuml

## Working with this repostiory

To use this repository, please us the `make help` command to see the available
commands. This project is still a proof-of-concept, but it attempts to
automate all the necessary parts to get started using the Structurizr DSL
locally with PNG files as an output.

## Development

> This section is still being written. Please read through the `Makefile` and
> the `scripts/` directory for the source code that powers this repository for
> more information. Contributions are welcome. Also, please reach out to
> @rogeruiz on Twitter for more direct engagement.

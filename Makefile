wmt_export_name = $(lastword $(subst /, ,$(CURDIR)))_export
wmt_render_name = $(lastword $(subst /, ,$(CURDIR)))_render

EXECUTABLES = plantuml watchman docker less rg
K := $(foreach exec,$(EXECUTABLES),\
        $(if $(shell command -v $(exec)),some string,$(error "No $(exec) executable in PATH")))

check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))

.PHONY: render
render: setup-watchman-trigger ## A target to render the diagram file(s).

.PHONY: clean
clean: ## A target to clean things up left behind from `make render`.
	watchman trigger-del . $(wmt_export_name)
	watchman trigger-del . $(wmt_render_name)
	@rm -rvf out/

.PHONY: debug
debug: ## A target to debug things.
	@less +F /usr/local/var/run/watchman/$(shell whoami)-state/log

# A target to setup the Watchman trigger in the current directory.
.PHONY: setup-watchman-trigger
setup-watchman-trigger:
	@# I don't have a great place to put this, but I only need to do
	@# this one time but it's a no-op every other time.
	@@watchman watch . 2>&1 > /dev/null

	@# Same with creating this `out/` directory that's used by the
	@# `scripts/*.sh` files.
	@mkdir -p out/

	@watchman -- trigger . $(wmt_export_name) '**/*.dsl' -- scripts/export.sh
	@watchman -- trigger . $(wmt_render_name) '**/*.puml' -- scripts/render.sh

.PHONY: help
help: ## Outputs this help message.
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


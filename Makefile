docker_image = docker_developer_environment

.PHONY : tests

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

always:

##@ Docker

interactive-developer-environment-in-docker: ## everything needed to run all targets
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

##@ Development

target/debug/anon-csv: always
	cargo build

target/release/anon-csv: always
	cargo build --release

lint: ## Run lints with cargo clippy
	cargo clippy

profile: target/release/anon-csv ## Profile code with valgrind and callgrind, linux only
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/anon-csv ## run hyperfine
	hyperfine '$<'

##@ Testing

journey-tests: target/debug/anon-csv ## run all journey tests
	./tests/stateless-journey.sh $<

continuous-journey-tests: ## run all journey tests, and rerun on change
	watchexec $(MAKE) journey-tests


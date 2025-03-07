SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo "Usage: make <target>"
	@echo "Targets:"
	@echo "  build                        Build the project"
	@echo "  clean                        Clean the project"
	@echo "  test                         Run the tests"
	@echo "  run                          Run the project"
	@echo "  help                         Show this help message"

rebuilt: clean build ## Rebuild the project

build: ## Build the project
	@echo "Building the project..."
	cargo build

clean: ## Clean the project
	@echo "Cleaning the project..."
	cargo clean

lint: ## Run the linter
	@rustup component add clippy > /dev/null 2>&1
	@echo "Running the linter..."
	cargo clippy

run: ## Run the project
	@echo "Running the project..."
	cargo run

fmt: ## Format the code
	@rustup component add rustfmt > /dev/null 2>&1
	@echo "Formatting the code..."
	cargo fmt

test: ## Run the tests
	@echo "Running the tests..."
	cargo test

bump: ## Bump the version
	@echo "Bumping the version..."
	@echo "Current version: $$(cargo pkgid | cut -d '#' -f 2)"

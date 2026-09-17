# Paths and configuration variables
CARGO        := cargo
DIR := src
BUILDER      := builder

.PHONY: all run build clean fmt

# Default target when you type 'make'
all: build

# Runs your host-side builder pipeline using the explicit path
build:
	cd $(DIR) && $(CARGO) run -p $(BUILDER)

# Alias for running the build pipeline
run: build

# Cleans up build artifacts for the whole workspace
clean:
	cd $(DIR) && $(CARGO) clean

fmt:
	cd $(DIR) && $(CARGO) fmt


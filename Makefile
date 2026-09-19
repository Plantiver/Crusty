# Paths and configuration variables
CARGO        := cargo
DIR          := src
BUILDER      := builder
KERNEL		 := kernel

.PHONY: all run build clean fmt part

# Default target when you type 'make'
all: build

# Runs your host-side builder pipeline normally
build:
	cd $(DIR) && $(CARGO) run -p $(BUILDER)

# Triggers the builder pipeline and passes the 'run' parameter to handle QEMU execution
run:
	cd $(DIR) && $(CARGO) run -p $(BUILDER) -- run

# Cleans up build artifacts for the whole workspace
clean:
	cd $(DIR) && $(CARGO) clean
	rm src/crusty.iso

fmt:
	cd $(DIR) && $(CARGO) fmt

part:
	cd $(DIR) && $(CARGO) build -p $(KERNEL) --target x86_64-unknown-none

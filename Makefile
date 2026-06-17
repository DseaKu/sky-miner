PROJECT_NAME = $(shell grep "config/name" godot/project.godot | cut -d'=' -f2 | tr -d '"')
EDITOR ?= nvim

# Detect OS
UNAME_S := $(shell uname -s)

# Detect Godot executable
ifeq ($(UNAME_S),Darwin)
    GODOT ?= $(shell which godot 2>/dev/null || echo "/Applications/Godot.app/Contents/MacOS/Godot")
    USER_DATA_DIR = $(HOME)/Library/Application Support/Godot/app_userdata/$(PROJECT_NAME)
else
    GODOT ?= $(shell which godot 2>/dev/null || which godot-engine 2>/dev/null || echo "godot")
    USER_DATA_DIR = $(HOME)/.local/share/godot/app_userdata/$(PROJECT_NAME)
endif

check:
	@ cargo check --manifest-path rust/Cargo.toml

clippy:
	@ cargo clippy --manifest-path rust/Cargo.toml

build:
	@ cargo build --manifest-path rust/Cargo.toml

run: 
	@"$(GODOT)" --path godot

conf:
	@$(EDITOR) "$(USER_DATA_DIR)"

clean-conf:
	@rm -rf "$(USER_DATA_DIR)"
	@echo "Cleaned all configs and data in $(USER_DATA_DIR)"


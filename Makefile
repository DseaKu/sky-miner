PROJECT_NAME = $(shell grep "config/name" godot/project.godot | cut -d'=' -f2 | tr -d '"')
EDITOR ?= nvim

# Detect OS for Godot user data path (macOS or Linux only)
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    USER_DATA_DIR = $(HOME)/Library/Application Support/Godot/app_userdata/$(PROJECT_NAME)
else
    USER_DATA_DIR = $(HOME)/.local/share/godot/app_userdata/$(PROJECT_NAME)
endif

check:
	@ cargo check --manifest-path rust/Cargo.toml

clippy:
	@ cargo clippy --manifest-path rust/Cargo.toml

build:
	@ cargo build --manifest-path rust/Cargo.toml

run: 
	@/Applications/Godot.app/Contents/MacOS/Godot --path godot

conf:
	@$(EDITOR) "$(USER_DATA_DIR)"

clean-conf:
	@rm -rf "$(USER_DATA_DIR)"
	@echo "Cleaned all configs and data in $(USER_DATA_DIR)"


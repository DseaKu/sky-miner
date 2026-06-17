UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
    GODOT ?= $(shell which godot 2>/dev/null || echo "/Applications/Godot.app/Contents/MacOS/Godot")
else
    GODOT ?= $(shell which godot 2>/dev/null || which godot-engine 2>/dev/null || echo "godot")
endif

run: 
	@"$(GODOT)" --path ../godot


check:
	@ cargo check --manifest-path rust/Cargo.toml

clippy:
	@ cargo clippy --manifest-path rust/Cargo.toml

build:
	@ cargo build --manifest-path rust/Cargo.toml

run: 
	@/Applications/Godot.app/Contents/MacOS/Godot --path godot

rbt = "RUST_BACKTRACE=1"
config = "./karabinux/tests/config/default.json"

# Builds the project.
build:
	cargo build

# Builds the project with `--features viewer`.
build-features-viewer:
	cargo build --features viewer

# Runs the project's tests.
test:
	cargo test --all

# Formats the project.
fmt:
	cargo fmt --all

# Checks the format of the project.
fmt-check:
	cargo fmt --all -- --check

# Runs karabinux with default config with the given device.
run device: build sudo
	sudo {{rbt}} ./target/debug/karabinux -g -d "{{device}}" -c "{{config}}"

# Same as `run`, but also activates the event viewer.
view device: build-features-viewer sudo
	sudo {{rbt}} ./target/debug/karabinux -v -g -d "{{device}}" -c "{{config}}"

# Publishes all the packages in the workspace.
publish: build build-features-viewer fmt-check test
	cargo publish --manifest-path ./Cargo.toml
	cargo publish --manifest-path ./karabinux/Cargo.toml

# Prompt for sudo (required by the emitter for `libevdev_uinput` devices).
sudo:
	sudo -v

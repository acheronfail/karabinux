RBT = "RUST_BACKTRACE=1"

# Runs the project's tests.
test:
	cargo test

# Builds the project.
build:
	cargo build

# Formats the project.
fmt:
	cargo fmt --all

# Runs `grabber -> mapper -> emitter` with the given device.
@run device config='./karabinux/tests/config/default.json': build sudo
	{{RBT}} ./target/debug/grabber -g -d "{{device}}" \
		| {{RBT}} ./target/debug/mapper -c {{config}} \
		| sudo {{RBT}} ./target/debug/emitter -d "{{device}}"

# Runs `grabber (with viewer) -> mapper -> emitter` with the given device.
@view device config='./karabinux/tests/config/default.json': build sudo
	{{RBT}} ./target/debug/grabber -v -g -d "{{device}}" \
		| {{RBT}} ./target/debug/mapper -c {{config}} \
		| sudo {{RBT}} ./target/debug/emitter -d "{{device}}"

# Runs `grabber -> emitter`.
@noop device: build sudo
	{{RBT}} ./target/debug/grabber -g -d "{{device}}" \
		| sudo {{RBT}} ./target/debug/emitter -d "{{device}}"

# Prompt for sudo (required by the emitter for `libevdev_uinput` devices).
@sudo:
	sudo -v

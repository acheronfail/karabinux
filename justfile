# Runs the project's tests.
test: build
	cargo test

# Builds the project.
build:
	cargo build

# Formats the project.
fmt:
	cargo fmt

# Runs `grabber -> mapper -> emitter` with the given device.
@run device config='./test/config/default.json': build sudo
	./target/debug/grabber -g -v "{{device}}" \
		| ./target/debug/mapper -c {{config}} \
		| sudo ./target/debug/emitter -v "{{device}}"

# Runs `grabber -> emitter`.
@noop device: build sudo
	./target/debug/grabber -g -v "{{device}}" | sudo ./target/debug/emitter -v "{{device}}"

# Prompt for sudo (required by the emitter for `libevdev_uinput` devices).
@sudo:
	sudo -v

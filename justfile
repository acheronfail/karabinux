# Runs the project's tests.
test: build
	cargo test

# Builds the project.
build:
	cargo build

# Formats the project.
fmt:
	cargo fmt

# Runs grabber -> mapper -> emitter with the given device.
@run device: build sudo
	./target/debug/grabber -g -v "{{device}}" \
		| ./target/debug/mapper -c ./test/config/config_1.json \
		| sudo ./target/debug/emitter -v "{{device}}"

# Runs grabber -> emitter.
@noop device: build sudo
	./target/debug/grabber -g -v "{{device}}" | sudo ./target/debug/emitter -v "{{device}}"

# Prompt for sudo (so it doesn't block other programs later on).
@sudo:
	sudo -v

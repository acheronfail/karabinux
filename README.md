# Karabinux

At attempt to port (as much as possible) the functionality of [Karabiner-Elements](https://github.com/tekezo/Karabiner-Elements) to Linux devices.

## Project structure

The project is divided into separate packages:

* `grabber`
	- Responsible for intercepting `libevdev` events from device file descriptors
* `mapper`
	- Responsible for mapping input events to output events (think of this as where the modifications occur)
* `emitter`
	- Responsible for emitting the mapped events to a virtual `libevdev_uinput` device
* `karabinux`
	- A shared set of utilities and configuration which parses Karabiner config and contains the remapping logic

## Using the project

A useful set of commands is found in the `justfile`, run `just --list` to see it.

## Development

In debug mode, the following shortcuts are available:

* `esc` -> writes several lines to `stderr` to clear the log
* `~` -> prints the parsed configuration file
* `backspace` (twice) -> terminates the `grabber` process
	- Since we grab libevdev devices, you can potentially get yourself into a state where your keyboard no longer works (after `grab`bing the device). This shortcut terminates the `grabber` process.

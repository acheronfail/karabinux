# Karabinux

At attempt to port (as much as possible) the functionality of [Karabiner-Elements](https://github.com/tekezo/Karabiner-Elements) to Linux devices.

## Project structure

The project is divided into separate packages:

* `grabber`
	- Responsible for intercepting `libevdev` events from device file descriptors
* `mapper`
	- Responsible for mapping input events to output events (think of this as where the modifications occur)
* `emitter`
	- Responsible for emitting the mapped events
* `karabinux`
	- A shared set of utilities and configuration

## Using the project

A common set of commands is found in the `justfile`.

## Development

Since we grab libevdev devices, you can potentially get yourself into a broken state where your keyboard no longer works (after `grab`bing the device). You can try terminating the program in debug mode by pressing the `backspace` key twice in a row.

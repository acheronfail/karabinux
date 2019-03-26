[![Build Status](https://travis-ci.org/acheronfail/karabinux.svg?branch=master)](https://travis-ci.org/acheronfail/karabinux)

# Karabinux

At attempt to port (as much as possible) the functionality of [Karabiner-Elements] to Linux.

**ATTENTION:** This is very much a work in progress - do not expect this to be in a functional state at the moment!

## Project structure

* [![crates.io](https://img.shields.io/crates/v/karabinux.svg)](https://crates.io/crates/karabinux) `karabinux_cli`:
	- The top level package provides the binary for `karabinux_cli`.
	- It also handles the connection to `libevdev` and `uinput` which is used for grabbing and emitting events.
* [![crates.io](https://img.shields.io/crates/v/karabinux_cli.svg)](https://crates.io/crates/karabinux_cli) `karabinux`:
	- This is the core library for `karabinux`.
	- It contains the core logic as well as utilities and structs for parsing Karabiner configuration files.

### How does it work?

Karabinux takes advantage of `libevdev` in order to "grab" key events and re-emit them.
Take for example, this short exerpt from the [`libevdev` documentation]:

> Where does `libevdev` sit?
> 
> `libevdev` is essentially a `read(2)`` on steroids for `/dev/input/eventX` devices. It sits below the process that handles input events, in between the kernel and that process. In the simplest case, e.g. an evtest-like tool the stack would look like this:
> 
> ```
>  kernel → libevdev → evtest
> ```
> 
> For X.Org input modules, the stack would look like this:
> 
> ```
>  kernel → libevdev → xf86-input-evdev → X server → X client
> ```
> 
> For Weston/Wayland, the stack would look like this:
> 
> ```
>  kernel → libevdev → Weston → Wayland client
> ```
> 
> libevdev does not have knowledge of X clients or Wayland clients, it is too low in the stack.

So, Karabinux diverts the normal flow of events, transforms them, and emits new events via a virtual `uinput` device:

```
BEFORE:
kernel -> libevdev -> everything else

AFTER:
kernel -> libevdev              libevdev_uinput -> everything else
                   \           /
                     karabinux
```

## Using the project

This project uses [`just`] to run commands.
A useful set of commands is found in the [`justfile`], run `just --list` to see it.

## Development

When running the project in debug mode, the following shortcuts are available:

* Press `backspace` twice in a row -> terminates the `grabber` process
	- Since we grab libevdev devices, you can potentially get yourself into a state where your keyboard no longer works (after `grab`bing the device). This shortcut terminates the `grabber` process.

If you want a graphical representation of the event mappings, you can use the `just view /dev/input/path/to/your/device` command. This creates a GTK window and displays the mappings live. 

## References & Related

* [`libevdev` documentation]
* [Linux Input documentation]
* [Karabiner]
	- [Karabiner documentation]
	- [Karabiner configuration reference]
* [Interception Tools]

## License

This library is dual-licensed under either of the following, at your option:

* [Apache License], Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* [MIT License] (LICENSE-MIT or http://opensource.org/licenses/MIT)

[`libevdev` documentation]: https://www.freedesktop.org/software/libevdev/doc/latest/index.html
[Linux Input documentation]: https://www.kernel.org/doc/html/v4.17/input/
[Karabiner-Elements]: https://github.com/tekezo/Karabiner-Elements
[Karabiner]: https://pqrs.org/osx/karabiner/
[Karabiner documentation]: https://pqrs.org/osx/karabiner/json.html
[Karabiner configuration reference]: https://pqrs.org/osx/karabiner/document.html
[Interception Tools]: https://gitlab.com/interception/linux/tools
[`just`]: https://github.com/casey/just
[`justfile`]: ./justfile
[Apache License]: ./LICENSE-APACHE
[MIT License]: ./LICENSE-MIT

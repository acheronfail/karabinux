
# Karabinux

At attempt to port (as much as possible) the functionality of [Karabiner-Elements] to Linux devices.

## Project structure

The project is divided into separate packages:

* `grabber`: Responsible for intercepting `libevdev` events from device file descriptors
* `emitter`: Responsible for emitting the mapped events to a virtual `libevdev_uinput` device
* `mapper`: Responsible for mapping input events to output events (this is where the modifications occur)
* `karabinux`: A shared set of utilities and configuration which parses Karabiner config and contains the remapping logic

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

## Using the project

This project uses [`just`] to run commands.
A useful set of commands is found in the `justfile`, run `just --list` to see it.

## Development

When running the project in debug mode, the following shortcuts are available:

* Press `backspace` twice in a row -> terminates the `grabber` process
	- Since we grab libevdev devices, you can potentially get yourself into a state where your keyboard no longer works (after `grab`bing the device). This shortcut terminates the `grabber` process.

If you want a graphical representation of the event mappings, you can use the `just view /dev/input/path/to/your/device` command. This creates a GTK window and displays the mappings live. 

## References

* [`libevdev` documentation]
* [Linux Input documentation]
* [Karabiner]
	- [Karabiner documentation]
	- [Karabiner config referenence]

[`libevdev` documentation]: https://www.freedesktop.org/software/libevdev/doc/latest/index.html
[Linux Input documentation]: https://www.kernel.org/doc/html/v4.17/input/
[Karabiner-Elements]: https://github.com/tekezo/Karabiner-Elements
[Karabiner]: https://pqrs.org/osx/karabiner/
[Karabiner documentation]: https://pqrs.org/osx/karabiner/json.html
[Karabiner configuration referenence]: https://pqrs.org/osx/karabiner/document.html
[`just`]: https://github.com/casey/just

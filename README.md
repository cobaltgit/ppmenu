# Power Profile Picker (PPP)

Power profile picker for window managers using D-Bus!

<img src="./assets/demo.gif" alt="Demonstration of ppp" width="720">

## Supported launchers

* [`dmenu`](https://tools.suckless.org/dmenu/)
* [`rofi`](https://github.com/davatorium/rofi)
* [`fuzzel`](https://codeberg.org/dnkl/fuzzel)

## Requirements

* `power-profiles-daemon` OR TLP >=1.9.0 (with `tlp-pd` running)
	* Exposes the `org.freedesktop.UPower.PowerProfiles` D-Bus object
* One of the above launchers

## Installation and usage

Download the binary from the [latest release](https://github.com/cobaltgit/ppp/releases/latest) for your platform and drop it anywhere!

Usage examples:

* Basic usage:
	```sh
	# start ppp using dmenu
	$ ./ppp -l dmenu
	```
* Pass additional args to the launcher
	```sh
	# start ppp using fuzzel with coloured prompt
	$ ./ppp -l fuzzel -a "--prompt-color=0047abff"
	```

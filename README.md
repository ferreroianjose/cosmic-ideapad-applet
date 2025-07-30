# Cosmic Ideapad Control

## Description

A simple applet for the [Cosmic Desktop Environment](https://system76.com/cosmic/) that controls hardware features exposed via the by the [ideapad laptop kernel module](https://github.com/torvalds/linux/blob/master/Documentation/ABI/testing/sysfs-platform-ideapad-laptop). Built for personal use.

## Features

- Battery conservation mode
- Fn-lock toggle
- Fan mode control
- USB "always on"
- Camera module power

## Installation

This project requires [just](https://crates.io/crates/just) for an easier installation. You can install it with the command:

```bash
cargo install just
```

To build and install the applet:

```bash
git clone https://github.com/ferreroianjose/cosmic-ideapad-applet
cd cosmic-ideapad-control
just install
```

To uninstall, run:

```bash
just uninstall
```

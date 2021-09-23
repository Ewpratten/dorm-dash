# dorm-dash
[![Build](https://github.com/Ewpratten/dorm-dash/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/dorm-dash/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/dorm-dash/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/dorm-dash/actions/workflows/clippy.yml)

This program runs permanently on a Raspberry Pi hooked up to a wall-mounted TV in my dorm room. The goal is to have a small app that shows me the time, weather, and recent notices from the city.

## Building for raspberry pi

```sh
cross build --release --target armv7-unknown-linux-gnueabihf
```
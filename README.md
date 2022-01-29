<p align="center">
  <strong><a href="#quickstart">Quick Start</a> | <a href="#contribute">Contribute</a> </strong>
</p>

# Qoi
### A cross-platform CLI tool for viewing QOI files

Qoi is a simple tool for working with the [QOI image format](https://qoiformat.org). It's cross-platform and written in Rust.

## Quickstart
<a name="quickstart"></a>

First, download [the latest release](https://github.com/AlexGustafsson/qoi/releases) for your architecture.

The tool can then be used to view a file like so:

```shell
qoi view image.qoi
```

## Table of contents

[Quickstart](#quickstart)<br/>
[Features](#features)<br />
[Installation](#installation)<br />
[Usage](#usage)<br />
[Metrics](#metrics)<br />
[Contributing](#contributing)

<a id="features"></a>
## Features

* Supports QOI according to the specification version 1.0, 2022.01.05
* Cross-platform image viewer

<a id="installation"></a>
## Installation

### Downloading a pre-built release

Download the latest release from [here](https://github.com/AlexGustafsson/qoi/releases).

### Build from source

Clone the repository.

```shell
git clone https://github.com/AlexGustafsson/qoi.git && cd qoi
```

Optionally check out a specific version.

```shell
git checkout v0.1.0
```

Build the exporter.

```shell
cargo build --release
```

## Usage
<a name="usage"></a>

```
A Quite OK Image Format viewer

USAGE:
    tool <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    info    Display information about the image
    view    View an image
```

Example:

```shell
qoi view qoi.qoi
qoi info qoi.qoi
```

## Contributing
<a name="contributing"></a>

Any help with the project is more than welcome. Note though that this project is mostly meant for learning Rust. It's not intended to grow into some fully-fledged toolset.

### Development

```shell
# Clone the repository
https://github.com/AlexGustafsson/qoi.git && cd qoi

# Build for the native platform
cargo build

# Produce release builds for all platforms
cargo build
```

You might need to install toolchains for cross-compiling.

```shell
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-unknown-darwin-musl
rustup target add x86_64-apple-darwin
```

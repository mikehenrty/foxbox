# FoxBox

[![Build Status](https://travis-ci.org/fxbox/foxbox.svg?branch=master)](https://travis-ci.org/fxbox/foxbox)
[![Coverage Status](https://coveralls.io/repos/github/fxbox/foxbox/badge.svg?branch=master)](https://coveralls.io/github/fxbox/foxbox?branch=master)
[![License](https://img.shields.io/badge/license-MPL2-blue.svg)](https://raw.githubusercontent.com/fxbox/foxbox/master/LICENSE)


## Technologies

### Rust

We're using Rust for the daemon/server. Currently v1.8.x nightly is required.

To determine which version of rust is being used, check the
[.travis.yml](https://github.com/fxbox/foxbox/blob/master/.travis.yml) file.

Look for these 2 lines near the top of the file:
```yaml
rust:
    - nightly-2016-03-07
```
You should then be able to then use:
```
multirust override nightly-2016-03-07
```
to get the same version of toolchain that's being used for the travis tests.

Incidently, nightly-2016-03-07 corresponds to:
```bash
$ rustc -V
rustc 1.9.0-nightly (998a6720b 2016-03-07)
```

It's recommended that you use [`multirust`](https://github.com/brson/multirust)
to install and switch between versions of Rust.

#### Build requirements

| Dependency   | Debian/Raspian        | Fedora          | Arch             | OS X (Homebrew) |
| ------------ | --------------------- | --------------- | ---------------- | --------------- |
| `libupnp`    | `libupnp-dev`         | `libupnp-devel` | `libupnp`        | `libupnp`       |
| `libssl`     | `libssl-dev`          | `?`             | via `base-devel` | `openssl`       |
| `libavahi`   | `libavahi-client-dev` | `?`             | `avahi`          | `n.a.`          |
| `libsqlite3` | `libsqlite3-dev`      | `?`             | `sqlite`         | `sqlite`        |

### Node

We're using Node to run Selenium tests. Currently v4.x LTS. We plan to stay on
stable LTS releases. It's recommended that you use
[`nvm`](https://github.com/creationix/nvm) to install and switch between
versions of Node.


## Target hardware

We're using the Raspberry Pi 2 as a prototyping target (ARMv7). The target
operating system is the latest Raspbian which is based on Debian 8.0 Jessie.


## Contributing

Note: We're in an iterative prototyping phase of the project. Things are moving
really fast so it may be easier to contribute when the dust starts to settle.
You've been warned. :shipit:

### Forks and feature branches

You should fork the main repo and create pull requests against feature branches
of your fork. If you need some guidance with this see:

 - https://guides.github.com/introduction/flow/
 - http://scottchacon.com/2011/08/31/github-flow.html


## Setup

```bash
$ git clone git@github.com:<username>/foxbox.git
$ cd foxbox
```

## Running the daemon

```bash
$ cargo run
```

To run with custom local host name (eg. foxbox.local):

```bash
$ cargo run -- -l foxbox
```

__NOTE:__ currently changing of host name is done via ```avahi-daemon``` and therefore supported only on Linux platform. To be able to change local host machine name user must be either included into ```netdev``` group or allow any other suitable user group to manage host name by adding the following policy to ```/etc/dbus-1/system.d/avahi-dbus.conf```:
```xml
<policy group="any_suitable_group_name">
  <allow send_destination="org.freedesktop.Avahi"/>
  <allow receive_sender="org.freedesktop.Avahi"/>
</policy>
```

Alternatively you can build the app without running it via:

```bash
$ cargo build
```

## Build time options
### Disable authentication
You may want to disable endpoints authentication to ease your development process. You can do that by removing `authentication` from the `default` feature in the `Cargo.toml` file.

```conf
[features]
default = []
authentication = []
```

## Rust tests

```bash
$ cargo test
```


## Selenium tests

You'll need to make sure you install the dependencies via:

```bash
$ npm install
```

Then you can run the Selenium tests via:

```bash
$ npm test
```

It is equal to:

```bash
$ BOX_LOCAL_NAME=localhost BOX_PORT=3000 npm test
```

## Cross compiling to ARM

There is no one solution for this. The process will be different depending on
your operating system. You may be able to build on a RPi, but the larger the
application gets, the slower and more painful this will be (not recommended).

### Linux

@fabricedesre has created a script to help compile a toolchain. So far it's
only been tested on Ubuntu but there's nothing ubuntu specific so that should
work just fine on any Linux.

 - https://github.com/fabricedesre/rustpi2

For an extensive write-up about cross compiling Rust programs see:

 - https://github.com/japaric/rust-cross


### Mac OS X

Cross compiling on Mac hasn't been documented. A PR is welcomed. :wink:


## Notes for Mac OS X

You'll need some dependencies installed to build.

``` bash
$ brew install openssl libupnp sqlite
```

This is required to build the openssl crate using homebrew's openssl:

``` bash
$ export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include/
```

Previous versions of these instructions described setting ```OPENSSL_INCLUDE_DIR```.
Make sure it is unset. In fact, an obsolete value may have been cached by cargo
which is fixed by ```cargo clean```.


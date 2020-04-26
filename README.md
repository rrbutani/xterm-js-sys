## xterm-js-sys

[![Build Status][ci]][actions] [![License: MIT][license-badge]][license] [![crates.io][crates-badge]][crates] [![API Docs][docs-badge]][docs]

Rust bindings for [xterm.js][xterm].

## Features

Currently this covers about half the [xterm.js API](https://github.com/xtermjs/xterm.js/blob/master/typings/xterm.d.ts).

This crate has two features:
   - `ext`: Adds some nicer, more rust-y functions on top of the core bindings; all the functions are in [this module](src/ext).
   - `tui-backend`: Provides an xterm.js backed backend for the [tui][tui] crate; located [here](src/tui).

This crate also does support the infrastructure [xterm.js][xterm] has for [addons](https://github.com/xtermjs/xterm.js#addons). Currently only the [xterm-addon-fit](https://github.com/xtermjs/xterm.js/tree/master/addons/xterm-addon-fit) package has [Rust bindings][fit-addon]. If you do end up making bindings for an [xterm.js][xterm] addon, feel free to send in a PR to update this list!

### xterm.js addons

First party addon packages:
  - [attach][attach]: Unimplemented!
  - [fit][fit]: [xterm-js-addon-fit-sys][fit-addon]
  - [search][search]: Unimplemented!
  - [serialize][serialize]: Unimplemented!
  - [unicode11][unicode11]: Unimplemented!
  - [web-links][web-links]: Unimplemented!
  - [webgl][webgl]: Unimplemented!

[attach]: https://www.npmjs.com/package/xterm-addon-attach
[fit]: https://www.npmjs.com/package/xterm-addon-fit
[search]: https://www.npmjs.com/package/xterm-addon-search
[serialize]: https://www.npmjs.com/package/xterm-addon-serialize
[unicode11]: https://www.npmjs.com/package/xterm-addon-unicode11
[web-links]: https://www.npmjs.com/package/xterm-addon-web-links
[webgl]: https://www.npmjs.com/package/xterm-addon-webgl

## Usage

Add this to your `Cargo.toml`:
```TOML
[dependencies]
xterm-js-sys = "4.5.0-alpha0"
```

And make sure that your bundler/JS package manager is set to grab the corresponding verison of the [xterm.js][xterm] package. The examples in this repo use [parcel][parcel] for which all that's needed is adding `xterm` to your [`package.json`](examples/package.json):
```JSON
  "dependencies": {
    "xterm": "^4.5.0"
  }
```

Make sure you also add the packages for any addons you're using; see our [examples' `package.json`](examples/package.json) for an example.

## Examples

This repo has a [few examples](examples) that show usage of the bindings, usage with the `ext` feature, and one use of the [tui][tui] backend.

To build these, enter the folder of the example you wish to run (i.e. [examples/basic](examples/basic)), and:
  - install the packages (`npm i` or `yarn install`)
  - run the watch script (`npm run watch` or `yarn run watch`)

It should (hopefully) just work! 🤞

These examples are also deployed [here][examples].

## Minimum Supported Rust Version (MSRV)

This crate is currently guaranteed to compile on stable Rust 1.43 and newer. We offer no guarantees that this will remain true in future releases but do promise to always support (at minimum) the latest stable Rust version and to document changes to the MSRV in the [changelog](CHANGELOG.md).

## Contributing

PRs are (very) welcome!

[ci]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Frrbutani%2Fxterm-js-sys%2Fbadge&style=for-the-badge
[license-badge]: https://img.shields.io/github/license/rrbutani/xterm-js-sys?color=orange&style=for-the-badge
[crates-badge]: https://img.shields.io/crates/v/xterm-js-sys?style=for-the-badge
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge

[actions]: https://github.com/rrbutani/xterm-js-sys/actions
[license]: https://opensource.org/licenses/MIT
[crates]: https://crates.io/crates/xterm-js-sys
[docs]: https://rrbutani.github.io/xterm-js-sys/docs

[examples]: https://rrbutani.github.io/xterm-js-sys/examples

[xterm]: https://github.com/xtermjs/xterm.js/
[tui]: https://github.com/fdehau/tui-rs
[parcel]: https://parceljs.org/

[fit-addon]: https://github.com/rrbutani/xterm-js-addon-fit-sys

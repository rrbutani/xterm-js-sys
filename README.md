## xterm-js-sys

[![Build Status][ci]][actions] [![License: MIT][license-badge]][license] [![crates.io][crates-badge]][crates] [![API Docs][docs-badge]][docs]

Rust bindings for [xterm.js][xterm].

## Features

Currently this covers most of the [xterm.js API](https://github.com/xtermjs/xterm.js/blob/master/typings/xterm.d.ts).

This crate has two features:
   - `ext`: Adds some nicer, more rust-y functions on top of the core bindings; all the functions are in [this module](src/ext).
   - `crossterm-support`: Provides a wrapper type that let's [`crossterm`][crossterm] use xterm.js as a backend (located [here][crossterm-support]). This enables xterm.js to be used with, for example,the [tui][tui] crate. Usually you won't have to enable this feature yourself; you _should_ be able to just use [`crossterm`][crossterm] and pass it a [`Terminal`].

This crate also does support the infrastructure [xterm.js][xterm] has for [addons](https://github.com/xtermjs/xterm.js#addons). It also lets you [define your own addons in Rust][addon-ext-docs], if you'd like. Currently only the [xterm-addon-fit](https://github.com/xtermjs/xterm.js/tree/master/addons/xterm-addon-fit) package has [Rust bindings][fit-addon]. If you do end up making bindings for an [xterm.js][xterm] addon (or your own addon in Rust), feel free to send in a PR to update this list!

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
xterm-js-sys = "4.6.0-alpha1"
```

And make sure that your bundler/JS package manager is set to grab the corresponding verison of the [xterm.js][xterm] package. The examples in this repo use [parcel][parcel] for which all that's needed is adding `xterm` to your [`package.json`][package.json]:
```JSON
  "dependencies": {
    "xterm": "^4.6.0"
  }
```

Make sure you also add the packages for any addons you're using; see our [examples' `package.json`][package.json] for an example.

The [xterm.js documentation](https://xtermjs.org/docs/) is a good reference for actual usage of the API; these bindings are usually one to one. Though, as of this writing, the xterm.js docs still correspond to version 4.4.

## Examples

This repo has a [few examples][examples-src] that show usage of the bindings, usage with the `ext` feature, and usage with [`crossterm`][crossterm] and the [`tui`][tui] crate.

To build these, enter the folder of the example you wish to run (i.e. [examples/basic][examples-src-basic]) and:
  - install the packages (`npm i` or `yarn install`)
  - run the watch script (`npm run watch` or `yarn run watch`)

It should (hopefully) just work! 🤞

These examples are also deployed [here][examples].

## Minimum Supported Rust Version (MSRV)

This crate is currently guaranteed to compile on stable Rust 1.43 and newer. We offer no guarantees that this will remain true in future releases but do promise to always support (at minimum) the latest stable Rust version and to document changes to the MSRV in the [changelog][changelog].

## Contributing

PRs are (very) welcome!

[ci]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Frrbutani%2Fxterm-js-sys%2Fbadge&style=for-the-badge&labelColor=505050&color=A0CB8D
[license-badge]: https://img.shields.io/github/license/rrbutani/xterm-js-sys?style=for-the-badge&logo=GNU&labelColor=505050&color=998DCB
[crates-badge]: https://img.shields.io/crates/v/xterm-js-sys?style=for-the-badge&logo=rust&labelColor=505050&color=CB8DA0
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&labelColor=505050&color=8DBFCB


[changelog]: https://github.com/rrbutani/xterm-js-sys/tree/master/CHANGELOG.md

[actions]: https://github.com/rrbutani/xterm-js-sys/actions
[license]: https://opensource.org/licenses/MIT
[crates]: https://crates.io/crates/xterm-js-sys
[docs]: https://rrbutani.github.io/xterm-js-sys/docs/xterm_js_sys

[crossterm-support]: https://github.com/rrbutani/xterm-js-sys/tree/master/src/crossterm_support/

[addon-ext-docs]: https://rrbutani.github.io/xterm-js-sys/docs/xterm_js_sys/ext/addon/trait.XtermAddon.html

[examples]: https://rrbutani.github.io/xterm-js-sys/examples
[examples-src]: https://github.com/rrbutani/xterm-js-sys/tree/master/examples
[examples-src-basic]: https://github.com/rrbutani/xterm-js-sys/tree/master/examples/basic
[package.json]: (https://github.com/rrbutani/xterm-js-sys/tree/master/examples/package.json)

[xterm]: https://github.com/xtermjs/xterm.js/
[crossterm]: https://github.com/crossterm-rs/crossterm
[tui]: https://github.com/fdehau/tui-rs
[parcel]: https://parceljs.org/

[fit-addon]: https://github.com/rrbutani/xterm-js-addon-fit-sys

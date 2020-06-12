## Examples

### Running These

As mentioned in the [repo README](../README.md), to build these, enter the folder of the example you wish to run (i.e. [basic](basic), and:
  - install the packages (`npm i` or `yarn install`)
  - run the watch script (`npm run watch` or `yarn run watch`)

If all goes well, it'll just work! 🤞

### The List

Right now we've got:
  - **[basic](basic/src/lib.rs)**: A simple demo that's almost identical to the ['fake terminal' part](https://github.com/xtermjs/xterm.js/blob/d8bc7ceaffe3e4b2fea076a342f807f0ae210de8/demo/client.ts#L228-L261) of the [xterm.js demo](https://github.com/xtermjs/xterm.js/tree/master/demo).
  - **[sparkline](tui/src/lib.rs)**: A copy of the [sparkline demo][sparkline] in the [`tui` crate][tui].

These examples are also deployed [here][examples].

[examples]: https://rrbutani.github.io/xterm-js-sys/examples

[sparkline]: https://github.com/fdehau/tui-rs/blob/3f62ce9c199bb0048996bbdeb236d6e5522ec9e0/examples/sparkline.rs
[tui]: https://github.com/fdehau/tui-rs/

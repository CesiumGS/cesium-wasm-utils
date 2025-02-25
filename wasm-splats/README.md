# wasm-splats

[![wasm-splats CI](https://github.com/CesiumGS/cesium-wasm-utils/actions/workflows/wasm-splats-ci.yml/badge.svg)](https://github.com/CesiumGS/cesium-wasm-utils/actions/workflows/wasm-splats-ci.yml)
[![npm](https://img.shields.io/npm/v/@cesium/wasm-splats)](https://www.npmjs.com/package/@cesium/wasm-splats)

The `wasm-splats` package contains high-performance algorithms used in the rendering of Gaussian Splats in CesiumJS.

## Getting Started

Follow the instructions in the [cesium-wasm-utils README](./README.md) to clone the repository and install
prerequisites.

### Building

To build the package, run:

```sh
wasm-pack build --release --target web --scope cesium
```

This will output a `pkg` directory containing the compiled WebAssembly module and JavaScript bindings.

### Testing

To run the unit and integration tests, run:

```sh
wasm-pack test --headless --chrome --firefox
```

In macOS, you can also add `--safari` to run the tests in Safari.

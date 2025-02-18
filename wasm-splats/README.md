# wasm-splats

The `wasm-splats` package contains high-performance algorithms used in the rendering Gaussian Splats in CesiumJS.

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

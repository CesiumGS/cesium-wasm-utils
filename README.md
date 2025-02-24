# cesium-wasm-utils

![Cesium](https://github.com/CesiumGS/cesium/wiki/logos/Cesium_Logo_Color.jpg)

CesiumJS is a JavaScript library for creating 3D globes and 2D maps in a web browser without a plugin. It uses WebGL for
hardware-accelerated graphics, and is cross-platform, cross-browser, and tuned for dynamic-data visualization.

Built on open formats, CesiumJS is designed for robust interoperability and scaling for massive datasets.

The `cesium-wasm-utils` mono-repository contains utilities for CesiumJS written in WebAssembly (Wasm) for
performance-critical tasks.

**NOTE**: This repository is only required for development of these WebAssembly packages. If you are a CesiumJS user or
contributor, you do not need to clone this repository. Instead, follow the instructions in
the [CesiumJS README](https://github.com/CesiumGS/cesium/blob/main/README.md).

## Packages in this Repository

- [wasm-splats](wasm-splats/README.md): High-performance algorithms used in the rendering of Gaussian Splats in
  CesiumJS.

# Get Started

These instructions assume that you already
have [CesiumJS](https://github.com/CesiumGS/cesium/blob/main/README.md#rocket-get-started) configured on your system.

## Prerequisites

- [Node.js](https://nodejs.org/en/download/) v22 or later.
- [Rust](https://www.rust-lang.org/tools/install) v1.55 or later.
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) v0.13 or later.

### Installation recommendations

#### Node.js

##### Windows

On Windows, we recommend using [chocolatey](https://chocolatey.org/) to install Node.js.

```sh
choco install nodejs
```

##### Linux and macOS

On Linux and macOS, we recommend using [nvm](https://github.com/nvm-sh/nvm) to install Node.js.

See the [nvm README](https://github.com/nvm-sh/nvm/blob/master/README.md) for installation instructions.

#### Rust

On all platforms, we recommend using [rustup](https://rustup.rs/) to install Rust.

See the [rust website](https://www.rust-lang.org/tools/install) for installation instructions.

#### wasm-pack

On all platforms, we recommend using the wasm-pack installer to install wasm-pack.

See the [wasm-pack website](https://rustwasm.github.io/wasm-pack/installer/) for installation instructions.

## Get the Code

### You have commit access to `cesium-wasm-utils`

Clone the repository:

```sh
git clone git@github.com:CesiumGS/cesium-wasm-utils.git
```

### You do not have commit access to `cesium-wasm-utils`

You need to fork `cesium-wasm-utils`:

1. Fork the repository on GitHub.
2. Clone your fork, e.g., `git clone git@github.com:yourusername/cesium.git`.
3. Make changes in a branch, e.g., `git checkout -b my-feature`.

## Generate Documentation and Open in Browser

To generate the documentation for all packages in the workspace and open in your default browser, run:

```sh
cargo doc --no-deps --document-private-items --open
```

## Further Instructions

For further instructions on building and running the packages in this repository, see the README in each package
directory.

# Contribution Guide

Thanks for contributing to Cesium WASM Utilities. You rock! Are you

- [submitting an issue](#submitting-an-issue),
- [getting started contributing](#getting-started-contributing), or
- [opening a pull request](#opening-a-pull-request)?

To ensure an inclusive community, contributors and users in the Cesium community should follow the [code of conduct](./CODE_OF_CONDUCT.md).

## Submitting an Issue

If you have a question, do not submit an issue; instead, search the [Cesium community forum](https://community.cesium.com/). The forum is very active and there are years of informative archives, often with answers from the core Cesium team. If you do not find an answer to your question, start a new thread, and you'll likely get a quick response.

If you think you've found a bug in Cesium WASM Utilities, first search the [issues](https://github.com/CesiumGS/cesium-wasm-utils/issues). If an issue already exists, please add a comment expressing your interest and any additional information. This helps us prioritize issues.

If a related issue does not exist, submit a new one. Please be concise and include as much of the following information as is relevant:

- Minimum amount of sample code (and data) shared through [Sandcastle](https://sandcastle.cesium.com).
- Screenshot or animated .gif if appropriate (try [LICEcap](http://www.cockos.com/licecap/)). For example, see [#3153](https://github.com/CesiumGS/cesium/issues/3153) in CesiumJS. Screenshots are particularly useful for exceptions and rendering artifacts. If it is a rendering artifact, also include the output of [webglreport.com](http://webglreport.com/).
- Link to the thread if this was discussed on the Cesium forum or elsewhere. For example, see [#3045](https://github.com/CesiumGS/cesium/issues/3045) in CesiumJS.
- Your operating system and version, browser and version, and video card. Are they all up-to-date? Is the issue specific to one of them?
- The version of CesiumJS. Did this work in a previous version?
- Ideas for how to fix or workaround the issue. Also mention if you are willing to help fix it. If so, the Cesium team can often provide guidance and the issue may get fixed more quickly with your help.

## Getting Started Contributing

Everyone is welcome to contribute to Cesium WASM Utilities!

In addition to contributing core Cesium WASM Utilities code, we appreciate many types of contributions:

- Being active on the [Cesium community forum](https://community.cesium.com/) by answering questions and providing input on Cesium's direction.
- Showcasing your Cesium apps on [Cesium blog](https://cesium.com/blog/categories/userstories/). Contact us at hello@cesium.com.
- Writing tutorials, creating examples, and improving the reference documentation.
- Submitting issues as [described above](#submitting-an-issue).
- Triaging issues. Browse the [issues](https://github.com/CesiumGS/cesium-wasm-utils/issues) and comment on issues that are no longer reproducible or on issues which you have additional information.
- Creating ecosystem projects for [glTF](https://github.com/KhronosGroup/glTF/issues/456), [CZML](https://github.com/CesiumGS/cesium/wiki/CZML-Guide), and [3D Tiles](https://github.com/CesiumGS/3d-tiles).

See the [README](README.md) for how to build and run on your system.

Always feel free to introduce yourself on the [Cesium community forum](https://community.cesium.com/) to brainstorm ideas and ask for guidance.

## Opening a Pull Request

We love pull requests. We strive to promptly review them, provide feedback, and merge. Interest in Cesium is at an all-time high so the core team is busy. Following the tips in this guide will help your pull request get merged quickly.

> If you plan to make a major change, please start a new thread on the [Cesium community forum](https://community.cesium.com/) first. Pull requests for small features and bug fixes can generally just be opened without discussion on the forum.

### Contributor License Agreement (CLA)

Before we can review a pull request, we require a signed Contributor License Agreement. There is a CLA for:

- [individuals](https://docs.google.com/forms/d/e/1FAIpQLScU-yvQdcdjCFHkNXwdNeEXx5Qhu45QXuWX_uF5qiLGFSEwlA/viewform) and
- [corporations](https://docs.google.com/forms/d/e/1FAIpQLSeYEaWlBl1tQEiegfHMuqnH9VxyfgXGyIw13C2sN7Fj3J3GVA/viewform).

This only needs to be completed once, and enables contributions to all of the projects under the [CesiumGS](https://github.com/CesiumGS) organization, including Cesium WASM Utilities. The CLA ensures you retain copyright to your contributions, and provides us the right to use, modify, and redistribute your contributions using the [Apache 2.0 License](LICENSE.md).

If you have any questions, feel free to reach out to [hello@cesium.com](mailto:hello@cesium)!

### Pull Request Guidelines

Our code is our lifeblood so maintaining Cesium WASM Utilities' high code quality is important to us.

- Review the [Contributor Guide](./CONTRIBUTING.md). In addition to Cesium WASM Utilities specific topics, they contain a lot of general software development best practices.
- For an overview of our workflow see [GitHub pull request workflows](https://cesium.com/blog/2013/10/08/github-pull-request-workflows/).
- Pull request tips
  - If your pull request fixes an existing issue, include a link to the issue in the description (like this: "Fixes [#1](https://github.com/CesiumGS/cesium/issues/1)"). Likewise, if your pull request fixes an issue reported on the Cesium forum, include a link to the thread.
  - If your pull request needs additional work, include a [task list](https://github.com/blog/1375%0A-task-lists-in-gfm-issues-pulls-comments).
  - Once you are done making new commits to address feedback, add a comment to the pull request such as `"this is ready"` since GitHub doesn't notify us about commits.
- Code and tests
  - Follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) and verify that your code passes [rustfmt](https://github.com/rust-lang/rustfmt).
  - Verify that your code passes [clippy](https://doc.rust-lang.org/clippy/index.html).
  - Verify that all tests pass, and write new tests with excellent code coverage for new code.
  - Update the [CHANGES.md](CHANGES.md) file with a summary of your changes.
  - If you added third-party libraries, including new version of existing libraries, update [LICENSE.md](LICENSE.md). Mention it in [CHANGES.md](CHANGES.md). If you plan to add a third-party library, start a [GitHub issue](https://github.com/CesiumGS/cesium-wasm-utils/issues/new) discussing it first.

### Code of Conduct

To ensure an inclusive community, contributors and users in the Cesium community should follow the [code of conduct](./CODE_OF_CONDUCT.md).

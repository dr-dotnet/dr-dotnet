# CI / CD

## Versioning

Dotnet uses `x.x.x.x` versioning, while Rust uses only `x.x.x`. As a product, we chose to use `x.x.x.x` for versioning Dr-Dotnet. The first three numbers are for the version, and the fourth number is reserved for the build number which is automatically incremented on every commit. Since this is automatic, tag for creating releases should only be `x.x.x`.

During a build, the dotnet solution will look for `VERSION` and `BUILD_NUMBER` environment variables.
- When triggering cargo build, `build.rs` will look for `VERSION` for versioning the rust binary
- Dotnet will look for `VERSION.BUILD_NUMBER` for versioning the C# libraries.

In the Dr-Dotnet UI, the version is displayed in the header. That version is taken from the managed assembly (so it includes the build number).

## How to test/experiment

Follow the [building guidelines](BUILDING.md) to build the project. Then you should already be able to test and experiment with things locally, either by starting the desktop or the web version.

Release candidates and other experimental versions can be pushed to Docker Hub by manually triggering a dev deployment on the `on-commit` workflow. This can only happen for non-master branches. The docker image will be tagged `x.x.x.build_id-branch_name` (for instance `0.145.23.1234-my-branch`).

## Release workflow

A release can be made automatically by creating a tag `x.x.x` from the master branch. This will automatically:
- Create a new release on github
- Create a new docker image tagged `x.x` (skip minor version) and `latest`
- Create a new desktop version in .zips for all platforms
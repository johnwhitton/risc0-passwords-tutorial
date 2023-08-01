# risc0-passwords-tutorial

Repo for ZK Hack III - 5 Checking Passwords (RISC Zero Live Coding Example)

## Overview

## Initialization

Following are the steps to create this repository

1. Create a blank repository with a README.md and use a rust ignore template.
2. Clone the repo e.g.

   ```bash
   git clone https://github.com/johnwhitton/risc0-passwords-tutorial.git
   ```

3. Follow the [Factors Tutoiral Create new Project](https://github.com/risc0/risc0/blob/main/examples/factors/README.md#step-1-create-a-new-project)

   ```bash
   cargo install cargo-risczero
   mkdir tmp
   cd tmp
   cargo risczero new risc0-passwords-tutorial
   cd ..
   cp -rf tmp/risc0-passwords-tutorial/* risc0-passwords-tutorial/.
   rm -rf tmp
   cd risc0-passwords-tutorial
   cargo run
   ```

## Building the Project

To build the project you can either follow the instructions in [FACTORS.md](./FACTORS.md) or follow the video from [ZK Hack III - 5 Checking Passwords (RISC Zero Live Coding Example)](https://youtu.be/Yg_BGqj_6lg?list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5).

In this repository

## References

- [Factors Tutorial](https://github.com/risc0/risc0/blob/main/examples/factors/README.md#step-1-create-a-new-project)
- [ZK Hack III - 5 Checking Passwords (RISC Zero Live Coding Example)](https://youtu.be/Yg_BGqj_6lg?list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5) : Youtube Video
- [zkVM Quick Start](https://dev.risczero.com/zkvm/quickstart)
- [Bonsai Quick Start](https://dev.risczero.com/bonsai/quickstart)

## Appendices

### Appendix A RISC Zero Rust Starter Template

The following markdown is generated when running `cargo risczero new risc0-passwords-tutorial` it has been incorporated into this README.md for reference.

#### A RISC Zero Rust Starter Template

Welcome to the RISC Zero Rust Starter Template! This template is intended to give you a starting point for building a project using the RISC Zero zkVM. Throughout the code are comments labelled `TODO` in places where we expect projects will need to modify the code.
To better understand the concepts behind this template, check out our [Understanding the Starter Template] explainer.

TODO: Replace this README with a README for your project
TODO: Verify whether the included `.gitignore`, `LICENSE`, and `rust-toolchain` files are appropriate to your project

#### Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```bash
cargo run
```

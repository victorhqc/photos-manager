# pictures-manager

## Introduction

I have a lot of pictures, many of them are duplicated, many live in the same path and they're a
complete mess, whenever I open the pictures folder, the explorer takes so much time to index them
all, which is less than ideal.

My main initial goal on this project is to add some utilities for myself to try to order them in
separate folders using a CLI. In the future more utilities will be added, and maybe a GUI will be
added as well.

Having a GUI in the future is the main reason to have two packages, the `cli` which will internally
will call the `core` package, which will have all the business logic which in a possible future will
be used by the GUI.

## Development

### Requirements

- git
- Rust >= 1.64.0

### Run CLI

```bash
cargo run -- --bin=cargo-manager-cli
```

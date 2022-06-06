# git-download

[![Crates.io](https://img.shields.io/crates/v/git-download.svg)](https://crates.io/crates/git-download)
[![documentation](https://docs.rs/git-download/badge.svg)](https://docs.rs/git-download)
![CI](https://github.com/akiradeveloper/git-download/workflows/CI/badge.svg)

Microservices architecture requires sharing service definition files like in protocol buffer, for clients to access the server.

To share the files, one can choose primitive copy & paste approach but this is too vulnerable to human mistakes.

The second and now widely accepted approach is [protodep](https://github.com/stormcat24/protodep) however, this isn't the best solution for Rust programmers

Because we, Rust programmers, want to download the files just like this in build.rs.

```rust
// build.rs

git_download::repo("https://github.com/akiradeveloper/lol")
    .branch_name("v0.9.1")
    .add_file("lol-core/proto/lol-core.proto", "proto/lol.proto")
    .exec()?;

tonic_build::configure()
    .build_server(false)
    .compile(&["lol.proto"], &["proto"])?;
```

## Usage

```
[build-dependencies]
git-download = "0.1"
```

## Implementation

Internally, git-download uses sparse-checkout to download designated files not all the repository.

The downloaded files are firstly put in temporary directory which is created by tempfile crate.
In Linux, the directory is created in tmpfs which is in-memory filesystem.
Therefore, no disk write occurs before copying the file in the tmpfs to the destination path. We can avoid writing the unchanged content by comparing the content before copying.
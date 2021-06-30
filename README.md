![https://crates.io/crates/libpefile](https://img.shields.io/crates/v/libpefile?style=plastic)
[![Build-and-Test](https://github.com/janstarke/libpefile/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/janstarke/libpefile/actions/workflows/build-and-test.yml)

# libpefile

library to parse PE files

## Installation

```
cargo install libpefile
```

## Usage example

```rust
use libpefile::*;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dll_file = PathBuf::from(format!("{}/samples/msaudite.dll", manifest_dir));
    let pefile = PEFile::new(dll_file)?;

    for msg in pefile.messages_iter()?.filter_map(|r| r.ok()) {
        println!("{}: '{}'", msg.msg_id, msg.text);
    }
    Ok(())
}
```

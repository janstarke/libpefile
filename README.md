![https://crates.io/crates/libpefile](https://img.shields.io/crates/v/libpefile?style=plastic)

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

    for msg in pefile.messages_iter()? {
        println!("{}: '{}'", msg.msg_id, msg.text);
    }
    Ok(())
}
```

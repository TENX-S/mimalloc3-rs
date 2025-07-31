## Usage

```rust
use mimalloc3_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Requirements

__C/C++__ compiler and __CMake__ is required for building [mimalloc](https://github.com/microsoft/mimalloc) with cargo.

# OBWIO

OBWIO is a Rust crate that provides low-level bindings to the OpenCL API, enabling direct access to GPU compute functionality across platforms. It is designed for developers who want full control over OpenCL without relying on high-level abstractions or wrappers.

## Overview

OBWIO exposes the raw OpenCL interface through auto-generated Rust bindings. These bindings allow you to interact with OpenCL devices, contexts, buffers, and kernels directly, making it suitable for performance-critical applications such as graphics engines, machine learning frameworks, and custom GPU runtimes.

## Features

- Auto-generated bindings using `bindgen`
- Supports OpenCL 1.x and 2.x
- Cross-platform compatibility (Windows, Linux, macOS)
- Minimal overhead and full control over OpenCL calls
- Ideal for advanced users building low-level GPU systems

## Installation

Add OBWIO to your `Cargo.toml`:

```toml
[dependencies]
obwio = "0.1"
```

## Example

```rust
use obwio::clGetPlatformIDs;

fn main() {
    unsafe {
        let mut num_platforms = 0;
        let err = clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms);
        println!("Found {} OpenCL platforms (err code: {})", num_platforms, err);
    }
}
```


## OBRAH
If you don't want to go through the trouble of pure bindings, use OBRAH.
OBRAH is built on OBWIO, and provides a high-level interface for OpenCL in Rust.

## Requirements

- Rust 1.70 or later
- OpenCL SDK installed on your system
- Compatible OpenCL runtime and drivers


## License

Licensed under:
- Apache License, Version 2.0

## Contributing

Contributions are welcome. If you're building a wrapper or using OBWIO in a larger project, feel free to open issues or submit pull requests.

## Acknowledgments

OBWIO uses `bindgen` to generate FFI bindings and depends on the Khronos OpenCL headers. Thanks to the Rust and OpenCL communities for their documentation and tooling support.

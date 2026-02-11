# draco_decoder 

`draco_decoder` is a Rust library for decoding Draco compressed meshes. It provides native and WebAssembly (WASM) support with efficient bindings to the official Draco C++ library.

## Overview

- **Native:**  
  The native part uses [`cxx`](https://cxx.rs/) to create safe and ergonomic FFI bindings that directly connect to Draco's C++ decoding library. This allows efficient and zero-copy mesh decoding in native environments.

- **WASM:**  
  For WebAssembly targets, `draco_decoder` leverages the official Draco Emscripten build. It uses a JavaScript Worker to run the Draco decoder asynchronously, enabling non-blocking mesh decoding in the browser. The JavaScript implementation is available in a separate repository:  
  [https://github.com/jiangheng90/draco_decoder_js.git](https://github.com/jiangheng90/draco_decoder_js.git)

This design provides a unified Rust API while seamlessly switching between native and WASM implementations under the hood.

## build guide
- install essential for cpp develop (cmake, cpp compiler, ..) 
- cargo build 

now it has passed all lateast platform build. but I'm not sure how to make installation guide to fit all platform.

⚠️ Warning:
This crate currently work in progress, I have not tested on many devices of building.  now on windows it only support build on MSVC, and it may have some build issues. 

⚠️ Warning:
On wasm, due to the multi-threaded interaction between Rust and JS, encoded data will be copied once when transferring from Rust to JS. When multi-threaded Draco wasm completes decoding and passes it back to Rust, a second copy occurs. This process inevitably causes performance overhead. Since supporting SharedArrayBuffer in browser environments requires cross-origin isolation, currently this is the only viable solution.


## native/wasm usage

### async api

```rust
use draco_decoder::{DracoDecodeConfig, AttributeDataType, decode_mesh};

// some async wrapper

let mut config = DracoDecodeConfig::new(vertex_count, index_count);

// Add attributes to decode (dimention and data type)
config.add_attribute(dim, AttributeDataType::Float32);
config.add_attribute(dim, AttributeDataType::Float32);

// Your Draco-encoded binary mesh data
let data: &[u8] = /* your Draco encoded data here */;

// Asynchronously decode the mesh data
let buf = decode_mesh(data, &config).await;

// wrapper end
```

sync api
```rust
use draco_decoder::{DracoDecodeConfig, AttributeDataType, decode_mesh};

let mut config = DracoDecodeConfig::new(vertex_count, index_count);

// Add attributes to decode (dimention and data type)
config.add_attribute(dim, AttributeDataType::Float32);
config.add_attribute(dim, AttributeDataType::Float32);

// Your Draco-encoded binary mesh data
let data: &[u8] = /* your Draco encoded data here */;

// decode the mesh data
let buf = decode_mesh_sync(data, &config)
```

## Performance

The performance of draco_decoder has been measured under different environments:
| Environment            | Typical Decoding Time |
| ---------------------- | --------------------- |
| Native (Release Build) | 3 ms – 7 ms           |
| WebAssembly (WASM)     | 30 ms – 50 ms         |




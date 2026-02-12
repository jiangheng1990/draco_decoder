#pragma once
#include "draco_decoder/src/ffi.rs.h"
#include "rust/cxx.h"
#include <cstdint>
#include <vector>

rust::Vec<uint8_t> decode_point_cloud(rust::Slice<const uint8_t> data);

size_t decode_mesh_direct_write(const uint8_t *data, size_t data_len,
                                uint8_t *out_ptr, size_t out_len);

size_t debug_mesh_buffer_len(const uint8_t *data, size_t data_len);

// Cache API
uint64_t cache_mesh(rust::Slice<const uint8_t> data);

void release_mesh_cache(uint64_t handle);

// Mesh Config from Cache
bool get_mesh_config(uint64_t handle, MeshConfig &config);

// Decode to pre-allocated buffer
size_t decode_mesh_to_buffer(uint64_t handle, uint8_t *out_ptr, size_t out_len);
